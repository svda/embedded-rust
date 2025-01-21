use embassy_executor::Spawner;

use crate::DisplayResources;




#[embassy_executor::task]
pub async fn display(spawner: Spawner, r: DisplayResources) -> ! {
    loop {
    
    }
}
