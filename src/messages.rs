// yes a whole file for an enum cause i like it
// messages the worker thread sends to the gui
pub enum WorkerMsg {
    Log(String),
    Progress(f32),
    Done,
    Failed(String),
}
