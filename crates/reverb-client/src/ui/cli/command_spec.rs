use std::{collections::HashMap};

use reverb_core::failure::failure::{Failure, FailureType};
use anyhow::anyhow;

use crate::ui::cli::{cli_ui, command_spec::CommandCallType::Args};

pub(super) struct CommandSpec {
    nodes: HashMap<String, CommandSpecNode>,
}

struct CommandSpecNode {
    valid_aliases: Vec<String>,
    help: String,
    children: Vec<String>,
    parent: String,
    handler: Option<fn(&str) -> Result<(), Failure>>,
    call_type: CommandCallType,
}

#[derive(PartialEq)]
pub enum CommandCallType {
    NoArgs,
    Args,
    NotCallable,
}

impl CommandSpecNode {
    fn new(
        valid_aliases: Vec<&str>,
        help: String,
        handler: Option<fn(&str) -> Result<(), Failure>>,
        call_type: CommandCallType,
        parent: &str,
    ) -> CommandSpecNode {
        CommandSpecNode {
            valid_aliases: valid_aliases.into_iter().map(|s| s.to_string()).collect(),
            help,
            children: Vec::new(),
            handler,
            call_type,
            parent: parent.to_string(),
        }
    }

    fn call(
        &self,
        input: Vec<&str>,
        position: usize,
        command_spec: &CommandSpec,
    ) -> Result<(), Failure> {
        // global help handling, if the current node is help, handle it here since it is a special case.
        if self.valid_aliases.contains(&"help".to_string()) {
            self.print_help(command_spec, 1);
            return Ok(());
        }

        // normal command handling
        for child in &self.children {
            let node = command_spec.get(child).unwrap();
            for alias in &node.valid_aliases {
                if alias == input.get(position).unwrap_or(&"") {
                    return node.call(input, position + 1, command_spec);
                }
            }
        }
        self.handle(input, position, command_spec)?;
        Ok(())
    }

    /// Handle calling this command, only to be called on the correct node after traversing the command tree
    fn handle(&self, input: Vec<&str>, position: usize, _command_spec: &CommandSpec) -> Result<(), Failure> {
        let args;
        match self.call_type {
            CommandCallType::NoArgs => {
                if input.len() > position {
                    return Err(Failure::from((anyhow!("Command {} does not take arguments", input[..position].join(" ")), FailureType::Warning)));
                }
                args = String::new();
            }
            CommandCallType::Args => {
                if input.len() <= position {
                    return Err(Failure::from((anyhow!("Command {} requires arguments", input[..position].join(" ")), FailureType::Warning)));
                }
                args = input[position..input.len()].join(" ");
            }
            CommandCallType::NotCallable => {
                if input.len() <= position {
                    return Err(Failure::from((anyhow!("Command {} is not callable", input[..position].join(" ")), FailureType::Warning)));
                } else {
                    return Err(Failure::from((anyhow!("Command {} is not callable, unexpected arguments: {}", input[..position].join(" "), input[position..].join(" ")), FailureType::Warning)));
                } 
            }
        }
        if let Some(handler) = self.handler {
            handler(args.as_str())?;
            Ok(())
        } else {
            unreachable!("Command spec node {} has no handler, this should not be possible please report this bug", self.valid_aliases.get(0).unwrap_or(&"".to_string()));
        }
    }

    /// Print the help for this command, if num_layers is greater than 0, also print the help for the subcommands, if num_layers is 0, only print the immediate children without their help text, but indicate that they have more help available
    fn print_help(&self, command_spec: &CommandSpec, num_layers: usize) {
        let mut out_string = String::from("Help:\n");
        self.parent(command_spec).sprint_help(command_spec, num_layers, &mut out_string);
        cli_ui::show_text_in_right_third(&out_string);
    }

    /// Sprint help information recursively for this command and its children into out_string
    fn sprint_help(&self, command_spec: &CommandSpec, num_layers: usize, out_string: &mut String) {
        let mut prefix  = String::new();
        let mut current_parent = self.parent(command_spec);
        while current_parent != command_spec.root() {
            prefix = format!("{} {}", current_parent.valid_aliases.get(0).unwrap_or(&"".to_string()), prefix);
            current_parent = current_parent.parent(command_spec);
        }
        out_string.push_str(&format!("{} {}{}\n", prefix, self.valid_aliases.join(" | "), self.help));
        if num_layers > 0 {
            for child in &self.children {
                let node = command_spec.get(child).unwrap();
                node.sprint_help(command_spec, num_layers - 1, out_string);
            }
        } else if self.children.len() > 0 {
            out_string.push_str(&format!(
                "{} {} <args> | help (for more information on this command and its subcommands)\n",
                prefix,
                self.valid_aliases.join(" | ")
            ));
        }
    }

    /// Get the parent node for this node
    fn parent<'a>(&self, command_spec: &'a CommandSpec) -> &'a CommandSpecNode{
        command_spec.get(&self.parent).unwrap()
    }
}

impl PartialEq for CommandSpecNode {
    fn eq(&self, other: &Self) -> bool {
        self.valid_aliases == other.valid_aliases
            && self.parent == other.parent
    }
}

impl CommandSpec {
    pub fn new () -> CommandSpec {
        let mut command_spec = CommandSpec {
            nodes: HashMap::new(),
        };
        command_spec.nodes.insert(
            "root".to_string(),
            CommandSpecNode::new(
                vec![],
                "REVERB commands:".to_string(),
                None,
                CommandCallType::NotCallable,
                "root",
            ),
        );
        command_spec
    }

    /// Add a command to the command spec
    /// Args:
    /// - name: the name of the command, this is used for referencing the command when adding children, it is not used for matching user input, all names must be unique
    /// - valid_aliases: the valid aliases for the command, this is used for matching user input, at least one alias must be provided, aliases cannot contain spaces
    /// - help: the help string for the command, this is shown in the help menu for the command
    /// - handler: the function that is called when the command is called, this should be None for commands that are not directly callable
    /// - call_type: the type of the command, this determines how the command is called, ie how to handle input after this command, a callable command with args cannot have children
    /// - parent: the name of the parent command, if None, the command is added as a child of the root command, this is the name provided when adding the parent command, not any alias
    pub fn add(
        mut self,
        name: &str,
        valid_aliases: Vec<&str>,
        help: &str,
        handler: Option<fn(&str) -> Result<(), Failure>>,
        call_type: CommandCallType,
        parent: Option<&str>,
    ) -> CommandSpec {
        let name = name.to_string();

        // check inputs are valid
        if self.nodes.contains_key(&name) {
            unreachable!("Command spec node with name {} already exists, this should not be possible please report this bug", name);
        }
        if valid_aliases.len() == 0 {
            unreachable!("Command spec node must have at least one valid alias, this should not be possible please report this bug");
        }
        for alias in &valid_aliases {
            if alias.contains(' ') {
                unreachable!("Command spec node aliases cannot contain spaces, invalid alias: {}, this should not be possible please report this bug", alias);
            }
        }

        let add_help_child = call_type != Args && !valid_aliases.contains(&"help");// calculated here before moving values in node creation
        let node = CommandSpecNode::new(valid_aliases, help.to_string(), handler, call_type, parent.unwrap_or("root"));
        self.nodes.insert(name.clone(), node);
        match parent {
            Some(parent) => match self.nodes.get(&parent.to_string()) {
                Some(parent_node) => {
                    for child in &parent_node.children {
                        for alias in self.get(&name).unwrap().valid_aliases.iter() {
                            if self.nodes.get(child).unwrap().valid_aliases.contains(alias) {
                                unreachable!(
                                    "Parent node {} already has a child with alias {}, this should not be possible please report this bug",
                                    parent, alias
                                );
                            }
                        }
                    }
                    if parent_node.call_type == Args {
                        unreachable!("Parent node {} is an args command, it cannot have children, this should not be possible please report this bug",parent);
                    }
                    self.nodes.get_mut(&parent.to_string()).unwrap().children.push(name.clone());
                }
                None => {
                    unreachable!(
                        "Parent node {} not found when adding command spec node, this should not be possible please report this bug",
                        parent
                    )
                }
            },
            None => self.root_mut().children.push(name.clone()),
        }
        if add_help_child {
            // add a help child node 
            self = self.add(
                format!("{}_help", name).as_str(),
                vec!["help", "h"],
                format!(" : Show help for {} command", name).as_str(),
                None,
                CommandCallType::NoArgs,
                Some(name.as_str()),
            );
        }
        self
    }

    /// Call a command from suer input.
    /// in longer words this will: take the user input, parse it, 
    /// traverse the command tree to find the correct command spec node, 
    /// and then call the handler for that node with the remaining input as arguments
    pub fn call(&self, input: &str) -> Result<(), Failure> {
        let parts: Vec<&str> = input.split(' ').collect();
        self.root().call(parts, 0, &self)
    }

    fn get(&self, name: &str) -> Option<&CommandSpecNode> {
        self.nodes.get(name)
    }

    fn root_mut(&mut self) -> &mut CommandSpecNode {
        self.nodes.get_mut("root").unwrap()
    }

    fn root(&self) -> &CommandSpecNode {
        self.nodes.get("root").unwrap()
    }
}

