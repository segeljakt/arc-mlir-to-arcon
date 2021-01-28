//! An operator which executes Python code.

use arcon::prelude::*;
use arcon_macros::Arcon;
use arcon_state::Backend;
use kompact::prelude::ComponentDefinition;
use pyo3::prelude::*;

pub(crate) struct MyOperator {}

#[cfg_attr(feature = "arcon_serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Arcon, prost::Message, Copy, Clone, abomonation_derive::Abomonation)]
#[arcon(unsafe_ser_id = 12, reliable_ser_id = 13, version = 1)]
pub(crate) struct MyData {
    #[prost(double, tag = "1")]
    pub f: f64,
}

impl Operator for MyOperator {
    type IN = MyData;
    type OUT = MyData;
    type TimerState = ArconNever;
    type OperatorState = ();

    fn handle_element(
        &mut self,
        element: ArconElement<Self::IN>,
        mut ctx: OperatorContext<Self, impl Backend, impl ComponentDefinition>,
    ) -> OperatorResult<()> {
        let output = Python::with_gil(|py| my_python_handler(py, element.clone())).unwrap();

        for f in output {
            ctx.output(ArconElement {
                timestamp: element.timestamp,
                data: MyData { f },
            });
        }
        Ok(())
    }

    arcon::ignore_timeout!();
    arcon::ignore_persist!();
}

fn my_python_handler(py: Python, element: ArconElement<MyData>) -> PyResult<Vec<f64>> {
    PyModule::from_code(py, include_str!("./handler.py"), "handler.py", "handler")?
        .call("handle_element", (element.data.f,), None)?
        .extract()
}

// map(...)
//   .map(|asd| )
