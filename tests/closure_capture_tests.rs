
use std::ops::Deref;
use evilang_lib::interpreter::environment::Environment;
use evilang_lib::interpreter::runtime_values::PrimitiveValue;
use evilang_lib::interpreter::runtime_values::functions::Function;
use evilang_lib::interpreter::variables_containers::map::IVariablesMapConstMembers;

#[test]
fn test_closure_capture_optimization() {
    let code = r#"
    fn outer() {
        let unused = 100;
        let used = 200;
        fn inner() {
            return used;
        }
        return inner;
    }
    let closure = outer();
    "#;

    let mut env = Environment::new().unwrap();
    let _ = env.eval_program_string(code.to_string()).unwrap();

    // Get 'closure' variable
    let closure_var = env.scope.get_actual("closure".into()).expect("closure variable should exist");
    let closure_val = closure_var.borrow();

    match closure_val.deref() {
        PrimitiveValue::Function(func_ptr) => {
            match func_ptr.deref() {
                Function::Closure(closure) => {
                    // Check captured variables
                    let scope = &closure.parent_scope;
                    let vars = scope.variables.borrow();

                    // "used" should be captured
                    assert!(vars.contains_key("used".into()), "Captured scope should contain 'used'");

                    // "unused" should NOT be captured (memory leak check)
                    assert!(!vars.contains_key("unused".into()), "Captured scope should NOT contain 'unused'");

                    // Check value
                    let used_val = vars.get_actual("used".into()).unwrap();
                    assert_eq!(*used_val.borrow().deref(), PrimitiveValue::Number(200.into()));
                },
                _ => panic!("Expected a closure"),
            }
        },
        _ => panic!("Expected a function"),
    }
}
