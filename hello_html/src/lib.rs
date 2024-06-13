use  std::thread;
use std::sync::mpsc;
use  std::sync::Arc;
use std::sync::Mutex;


pub struct  ThreadPool{ 
    workers: Vec<Worker>,
    sender:mpsc::Sender<Message>,   //mpsc是一个多生产单消费的通道， 这里持有发送者。
}
//  智能指针Box ，为了编译时能知道类型的大小
//  type定义一个别名Job ，简化类型
type Job = Box<dyn FnOnce() + Send + 'static>;
enum Message {
    NewJob(Job),
    Terminate,
}
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,  //使用Option包装，是为了take移出thread的所有权，进行销毁，见impl Drop for ThreadPool
}



// Arc 是 原子RC  是可已在多线程中共享的指针（引用计数）
// Mutex 是一个互斥器，通过lock来取得对象所有权进行访问，  它实现了 Drop trait会在生命周期结束时释放unlock
// mpsc是一个多生产单消费的通道， receiver只有一个，想要在多线程中共享
impl Worker{
    fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Message>>>)->Worker{
        let thread = thread::spawn(move || loop { //使用loop考虑 锁的生命周期， 如果在方法体使用 while let Ok(job) = receiver.lock().unwrap().recv()            
                                                                  // 循环，因为Mutex没有pub的unlock方法，，它的unlock释放依赖生命周期结束，而while let不结束就不会unlock，会导致线程卡住
           let message = receiver.lock().unwrap().recv().unwrap();//当 let 语句结束时任何表达式中等号右侧使用的临时值都会立即被丢弃。然而 while let（if let 和 match）直到相关的代码块结束都不会丢弃临时值
           match message {
               Message::NewJob(job)=>{
                    println!("Worker {} got a job; execute...",id);
                    job();
                }
               Message::Terminate =>{
                println!("Worker {} was told to terminate.", id);

                    break;
               }
           }
        });
        Worker { id, thread:Some(thread)}
    }
}


impl ThreadPool{
    /// 创建一个新线程池
    /// 
    /// size 是线程池中的线程数量
    /// 
    /// # Panics
    /// 
    /// 当size <= 0, 这个 new 函数 将抛出panics
    pub fn new(size:usize)->ThreadPool{
        assert!(size>0);
        let (sender,receiver) = mpsc::channel();
        let  receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size  {
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }
        ThreadPool{workers,sender}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();

    }
}

impl Drop for ThreadPool{
    fn drop(&mut self) {
        println!("发送 Termintate 消息到所有线程。");
        for _ in &mut self.workers  {
            self.sender.send(Message::Terminate).unwrap();  
        }
        println!("停止所有的线程。");
       for work in &mut self.workers {
            if let Some(thread) =  work.thread.take(){
            println!("stop worker{}",work.id);
            thread.join().unwrap();
          }
       }
    }
}