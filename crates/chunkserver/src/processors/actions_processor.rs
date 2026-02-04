use common::master_channel::RequestedAction;

#[derive(Debug)]
pub struct ActionsProcessor;

impl ActionsProcessor {
    pub fn process(&mut self, requested_action: i32) {
        if requested_action == RequestedAction::ChunkListReport as i32 {
            self.process_chunk_list_report_action();
        }
    }

    fn process_chunk_list_report_action(&mut self) {

    }
}

