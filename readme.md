Rust library for running Oja's rule using multiple threads.

There are a couple of binaries prepared for testing different functionality contained inside this crate
These binaries have been configured to run with up to 32 threads. If you wish to run the examples with fewer threads, simply change the number of threads supplied to the thread pool in the desired bin-file.
MNIST data needs to be placed like this: `data/t10k-images-idx3-ubyte`


`cargo run --bin mnist_loading`
This command will load the MNIST dataset

`cargo run --bin oja_test`
This command will load the MNIST dataset, and train some neurons using oja's rule

`cargo run --bin plotting_testing`
This command will plot the CPU usage while running Oja's rule on mulitple threads


`cargo run --bin mt_network_benchmark`
This will run a benchmark running Oja's rule with a given number of threads. This can take over an hour.


`cargo run --bin thread_pool_test`
This will simply run a simple thread pool

