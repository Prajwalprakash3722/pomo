use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

enum InteractionEvent{
    Start,
    Pause,
    Stop
}

struct TimerThreadClient{
    interaction_sender: Sender<InteractionEvent>
}

enum TimerPlayState{
    Playing,
    Paused,
    Stopped
}
struct TimerThreadServer{
    
}

impl TimerThreadServer{
    pub fn launch()->(TimerThreadClient,JoinHandle<()>){
        let (interaction_channel_sender,interaction_channel_recv) = std::sync::mpsc::channel();
        
        let join_handle = std::thread::spawn(||{
            
        });
        
        (TimerThreadClient{interaction_sender:interaction_channel_sender},join_handle)
    }
}