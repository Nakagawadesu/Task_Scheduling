
#[derive(Clone)]

pub(crate) struct WorkerAnt {
    pub(crate) free_at: i128
}

impl WorkerAnt {
    pub fn new(start_time: i128) -> Self {
        Self {
            free_at: start_time
        }
    }
    //manusear ocupaçao
    pub fn start_task(&mut self ,elapsed : &i128 , cost : &i128  ){
        self.free_at = elapsed + cost ;
    }
}