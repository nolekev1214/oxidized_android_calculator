use std::thread;

use crossbeam::channel::Receiver;
use crossbeam::channel::Sender;
use j4rs::jni_sys::jint;
use j4rs::jni_sys::JavaVM;
use j4rs::prelude::*;
use j4rs::InvocationArg;
use j4rs_derive::*;
use lazy_static::lazy_static;

enum Operation {
    Add,
    Multiply,
    Divide,
    Subtract,
}

struct Calculators {
    pub operand_1: String,
    pub operand_2: String,
    pub operation: Operation,
    pub respond_location: Sender<f64>,
}

lazy_static! {
    static ref CHANNELS: (Sender<Calculators>, Receiver<Calculators>) =
        crossbeam::channel::bounded(10);
}

const JNI_VERSION_1_6: jint = 0x00010006;
#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn jni_onload(env: *mut JavaVM, _reserved: jobject) -> jint {
    j4rs::set_java_vm(env);
    JNI_VERSION_1_6
}

#[call_from_java("com.example.calculatorapp.MainActivity.startCalculatorService")]
fn start_service() {
    thread::spawn(|| channel_monitor());
}

#[call_from_java("com.example.calculatorapp.MainActivity.addInputs")]
fn add_inputs(i1: Instance, i2: Instance) -> Result<Instance, String> {
    let jvm = Jvm::attach_thread().unwrap();

    let operand_1: String = jvm.to_rust(i1).unwrap();
    let operand_2: String = jvm.to_rust(i2).unwrap();

    math_operation(operand_1, operand_2, Operation::Add)
}

#[call_from_java("com.example.calculatorapp.MainActivity.multiplyInputs")]
fn multiply_inputs(i1: Instance, i2: Instance) -> Result<Instance, String> {
    let jvm = Jvm::attach_thread().unwrap();

    let operand_1: String = jvm.to_rust(i1).unwrap();
    let operand_2: String = jvm.to_rust(i2).unwrap();

    math_operation(operand_1, operand_2, Operation::Multiply)
}

#[call_from_java("com.example.calculatorapp.MainActivity.divideInputs")]
fn divide_inputs(i1: Instance, i2: Instance) -> Result<Instance, String> {
    let jvm = Jvm::attach_thread().unwrap();

    let operand_1: String = jvm.to_rust(i1).unwrap();
    let operand_2: String = jvm.to_rust(i2).unwrap();

    math_operation(operand_1, operand_2, Operation::Divide)
}

#[call_from_java("com.example.calculatorapp.MainActivity.subtractInputs")]
fn subtract_inputs(i1: Instance, i2: Instance) -> Result<Instance, String> {
    let jvm = Jvm::attach_thread().unwrap();

    let operand_1: String = jvm.to_rust(i1).unwrap();
    let operand_2: String = jvm.to_rust(i2).unwrap();

    math_operation(operand_1, operand_2, Operation::Subtract)
}

fn math_operation(
    operand_1: String,
    operand_2: String,
    operation: Operation,
) -> Result<Instance, String> {
    let (tx, rx) = crossbeam::channel::bounded(1);

    let message = Calculators {
        operand_1,
        operand_2,
        operation,
        respond_location: tx,
    };

    _ = CHANNELS.0.send(message);

    let result = rx.recv().unwrap();

    let ia = InvocationArg::try_from(result).unwrap();
    Instance::try_from(ia).map_err(|error| format!("{}", error))
}

fn channel_monitor() {
    while let Ok(values) = CHANNELS.1.recv() {
        let out = match &values.operation {
            Operation::Add => add_strings(&values.operand_1, &values.operand_2),
            Operation::Subtract => subtract_strings(&values.operand_1, &values.operand_2),
            Operation::Divide => divide_strings(&values.operand_1, &values.operand_2),
            Operation::Multiply => multiply_strings(&values.operand_1, &values.operand_2),
        };
        _ = values.respond_location.send(out);
    }
}

fn add_strings(in1: &str, in2: &str) -> f64 {
    in1.parse::<f64>().unwrap_or(0.0) + in2.parse::<f64>().unwrap_or(0.0)
}

fn subtract_strings(in1: &str, in2: &str) -> f64 {
    in1.parse::<f64>().unwrap_or(0.0) - in2.parse::<f64>().unwrap_or(0.0)
}

fn divide_strings(in1: &str, in2: &str) -> f64 {
    in1.parse::<f64>().unwrap_or(1.0) / in2.parse::<f64>().unwrap_or(1.0)
}

fn multiply_strings(in1: &str, in2: &str) -> f64 {
    in1.parse::<f64>().unwrap_or(1.0) * in2.parse::<f64>().unwrap_or(1.0)
}
