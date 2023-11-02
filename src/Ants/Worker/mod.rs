
#[derive(Clone)]

pub(crate) struct WorkerAnt {
    pub(crate) free_at: i128,
    pub(crate) current_task: i128
}

impl WorkerAnt {
    pub fn new(start_time: i128) -> Self {
        Self {
            free_at: start_time,
            current_task: -1
        }
    }
    //manusear ocupaçao
    pub fn start_task(&mut self ,elapsed : &i128 , task : &i128 ,cost : &i128  ){
        if *task == 0
        {
            self.free_at = -1 ;
        }else{
            self.free_at = elapsed + cost ;
            self.current_task = *task; 
        }
        //println!("free at : {}",self.free_at);
    }
}