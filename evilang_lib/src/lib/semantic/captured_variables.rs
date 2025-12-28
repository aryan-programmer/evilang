use std::collections::HashSet;

use crate::ast::expression::{
	Expression,
	IdentifierT,
	MemberIndexer,
};
use crate::ast::statement::{ Statement, StatementList };
use crate::ast::structs::{
	FunctionDeclaration,
};

pub fn analyze_captured_variables(
	function_decl: &FunctionDeclaration
) -> HashSet<IdentifierT> {
	let mut analyzer = CapturedVariablesAnalyzer::new(function_decl);
	analyzer.analyze();
	return analyzer.captured_variables;
}

struct CapturedVariablesAnalyzer<'a> {
	root_function: &'a FunctionDeclaration,
	captured_variables: HashSet<IdentifierT>,
	scope_stack: Vec<HashSet<IdentifierT>>,
}

impl<'a> CapturedVariablesAnalyzer<'a> {
	fn new(root_function: &'a FunctionDeclaration) -> Self {
		let mut root_scope = HashSet::new();
		for param in &root_function.parameters {
			root_scope.insert(param.identifier.clone());
		}
		Self {
			root_function,
			captured_variables: HashSet::new(),
			scope_stack: vec![root_scope],
		}
	}

	fn analyze(&mut self) {
        self.collect_hoisted_declarations(&self.root_function.body);
		self.visit_statement(&self.root_function.body);
	}

    fn push_scope(&mut self) {
        self.scope_stack.push(HashSet::new());
    }

    fn pop_scope(&mut self) {
        self.scope_stack.pop();
    }

    fn declare_variable(&mut self, name: IdentifierT) {
        if let Some(top) = self.scope_stack.last_mut() {
            top.insert(name);
        }
    }

    fn is_variable_declared(&self, name: &IdentifierT) -> bool {
        for scope in self.scope_stack.iter().rev() {
            if scope.contains(name) {
                return true;
            }
        }
        return false;
    }

    fn collect_hoisted_declarations(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VariableDeclarations(decls) => {
                for decl in decls {
                    self.declare_variable(decl.identifier.clone());
                }
            }
            Statement::FunctionDeclarationStatement(fdecl) => {
                self.declare_variable(fdecl.name.clone());
            }
            Statement::ClassDeclarationStatement(cdecl) => {
                self.declare_variable(cdecl.name.clone());
            }
            Statement::BlockStatement(_stmts) => {
            }
            _ => {}
        }
    }

    fn collect_hoisted_declarations_list(&mut self, list: &StatementList) {
        for stmt in list {
            self.collect_hoisted_declarations(stmt);
        }
    }

	fn visit_statement_list(&mut self, list: &StatementList) {
		for stmt in list {
			self.visit_statement(stmt);
		}
	}

	fn visit_statement(&mut self, stmt: &Statement) {
		match stmt {
			Statement::BlockStatement(list) => {
                self.push_scope();
                self.collect_hoisted_declarations_list(list);
                self.visit_statement_list(list);
                self.pop_scope();
            }
			Statement::ExpressionStatement(expr) => self.visit_expression(expr),
			Statement::ReturnStatement(opt_expr) => {
				if let Some(expr) = opt_expr {
					self.visit_expression(expr);
				}
			}
			Statement::VariableDeclarations(decls) => {
				for decl in decls {
					if let Some(init) = &decl.initializer {
						self.visit_expression(init);
					}
				}
			}
			Statement::IfStatement { condition, if_branch, else_branch } => {
				self.visit_expression(condition);
				self.visit_statement(if_branch);
				if let Some(else_b) = else_branch {
					self.visit_statement(else_b);
				}
			}
			Statement::WhileLoop { condition, body } => {
				self.visit_expression(condition);
				self.visit_statement(body);
			}
			Statement::DoWhileLoop { condition, body } => {
				self.visit_expression(condition);
				self.visit_statement(body);
			}
			Statement::ForLoop { initialization, condition, increment, body } => {
                self.push_scope();
                self.collect_hoisted_declarations(initialization);

				self.visit_statement(initialization);
				self.visit_expression(condition);
				self.visit_statement(increment);
				self.visit_statement(body);

                self.pop_scope();
			}
			Statement::FunctionDeclarationStatement(func_decl) => {
                self.push_scope(); // Inner function scope
                for param in &func_decl.parameters {
                    self.declare_variable(param.identifier.clone());
                }
                self.collect_hoisted_declarations(&func_decl.body);
                self.visit_statement(&func_decl.body);
                self.pop_scope();
            }
            Statement::ClassDeclarationStatement(class_decl) => {
                 for method in &class_decl.methods {
                    self.push_scope();
                    for param in &method.parameters {
                        self.declare_variable(param.identifier.clone());
                    }
                    self.collect_hoisted_declarations(&method.body);
                    self.visit_statement(&method.body);
                    self.pop_scope();
                 }
            }
			Statement::NamespaceStatement { body, .. } => {
                self.push_scope();
                self.collect_hoisted_declarations_list(body);
                self.visit_statement_list(body);
                self.pop_scope();
            }
			Statement::ImportStatement { file_name, .. } => {
                self.visit_expression(file_name);
            }
			Statement::EmptyStatement => {}
			Statement::BreakStatement(_) => {}
			Statement::ContinueStatement(_) => {}
		}
	}

	fn visit_expression(&mut self, expr: &Expression) {
        match expr {
            Expression::Identifier(name) => {
                if !self.is_variable_declared(name) {
                    self.captured_variables.insert(name.clone());
                }
            }
            Expression::BinaryExpression { left, right, .. } |
            Expression::AssignmentExpression { left, right, .. } => {
                self.visit_expression(left);
                self.visit_expression(right);
            }
            Expression::UnaryExpression { argument, .. } => {
                self.visit_expression(argument);
            }
            Expression::ParenthesizedExpression(expr) => self.visit_expression(expr),
            Expression::FunctionCall(call) | Expression::NewObjectExpression(call) => {
                self.visit_expression(&call.callee);
                for arg in &call.arguments {
                    self.visit_expression(arg);
                }
            }
            Expression::MemberAccess { object, member } => {
                self.visit_expression(object);
                if let MemberIndexer::SubscriptExpression(expr) = member {
                    self.visit_expression(expr);
                }
            }
            Expression::FunctionExpression(func_decl) => {
                 self.push_scope();
                for param in &func_decl.parameters {
                    self.declare_variable(param.identifier.clone());
                }
                self.collect_hoisted_declarations(&func_decl.body);
                self.visit_statement(&func_decl.body);
                self.pop_scope();
            }
            Expression::ClassDeclarationExpression(class_decl) => {
                  for method in &class_decl.methods {
                    self.push_scope();
                    for param in &method.parameters {
                        self.declare_variable(param.identifier.clone());
                    }
                    self.collect_hoisted_declarations(&method.body);
                    self.visit_statement(&method.body);
                    self.pop_scope();
                 }
            }
             Expression::DottedIdentifiers(dotted) => {
                 if let Some(first) = dotted.identifiers.first() {
                      if !self.is_variable_declared(first) {
                        self.captured_variables.insert(first.clone());
                    }
                 }
             }
            _ => {}
        }
    }
}
