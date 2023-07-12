package com.example.calculatorapp;

import androidx.appcompat.app.AppCompatActivity;

import android.os.Bundle;
import android.view.View;
import android.widget.EditText;
import android.widget.TextView;

import org.astonbitecode.j4rs.api.Instance;
import org.astonbitecode.j4rs.api.java2rust.Java2RustUtils;

public class MainActivity extends AppCompatActivity {

    static {
        System.loadLibrary("calculator");
    }

    private static native void startCalculatorService();
    private static native Instance addInputs(Instance<String> s1, Instance<String> s2);
    private static native Instance multiplyInputs(Instance<String> s1, Instance<String> s2);
    private static native Instance divideInputs(Instance<String> s1, Instance<String> s2);
    private static native Instance subtractInputs(Instance<String> s1, Instance<String> s2);

    private EditText mOperandEditText1;
    private EditText mOperandEditText2;
    private TextView mResultTextView;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        mOperandEditText1 = findViewById(R.id.first_operand_edittext);
        mOperandEditText2 = findViewById(R.id.second_operand_edittext);
        mResultTextView = findViewById(R.id.result_textview);

        startCalculatorService();
    }

    public void sendMultiplyOperation(View view) {
        String[] ops = getOperands();
        Instance instance = multiplyInputs(Java2RustUtils.createInstance(ops[0]), Java2RustUtils.createInstance(ops[1]));
        setResultString(Java2RustUtils.getObjectCasted(instance));
    }

    public void sendDivideOperation(View view) {
        String[] ops = getOperands();
        Instance instance = divideInputs(Java2RustUtils.createInstance(ops[0]), Java2RustUtils.createInstance(ops[1]));
        setResultString(Java2RustUtils.getObjectCasted(instance));
    }

    public void sendSubtractOperation(View view) {
        String[] ops = getOperands();
        Instance instance = subtractInputs(Java2RustUtils.createInstance(ops[0]), Java2RustUtils.createInstance(ops[1]));
        setResultString(Java2RustUtils.getObjectCasted(instance));
    }

    public void sendAddOperation(View view) {
        String[] ops = getOperands();
        Instance instance = addInputs(Java2RustUtils.createInstance(ops[0]), Java2RustUtils.createInstance(ops[1]));
        setResultString(Java2RustUtils.getObjectCasted(instance));
    }

    private void setResultString(Double result) {
        String out = "Result: " + result;
        mResultTextView.setText(out);
    }

    private String[] getOperands() {
        String[] output = new String[2];
        output[0] = mOperandEditText1.getText().toString();
        output[1] = mOperandEditText2.getText().toString();

        return output;
    }
}