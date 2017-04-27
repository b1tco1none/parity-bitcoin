initSidebarItems({"enum":[["InitError","Custom error type for the rayon thread pool configuration."]],"fn":[["dump_stats","This is a debugging API not really intended for end users. It will dump some performance statistics out using `println`."],["initialize","Initializes the global thread pool. This initialization is **optional**.  If you do not call this function, the thread pool will be automatically initialized with the default configuration. In fact, calling `initialize` is not recommended, except for in two scenarios:"],["join","The `join` function takes two closures and potentially runs them in parallel but is not guaranteed to. However, the call to `join` incurs low overhead and is much different compared to spawning two separate threads."]],"mod":[["par_iter","The `ParallelIterator` module makes it easy to write parallel programs using an iterator-style interface. To get access to all the methods you want, the easiest is to write `use rayon::prelude::*;` at the top of your module, which will import the various traits and methods you need."],["prelude","The rayon prelude imports the various `ParallelIterator` traits. The intention is that one can include `use rayon::prelude::*` and have easy access to the various traits and methods you will need."]],"struct":[["Configuration","Contains the rayon thread pool configuration."],["ThreadPool",""]]});