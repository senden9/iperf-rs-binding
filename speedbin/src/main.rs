use iperf_sys::{iperf_free_test, iperf_new_test, iperf_test};

fn main() {
    unsafe {
        let test: *mut iperf_test = dbg!(iperf_new_test());
        if test.is_null() {
            panic!("Failed to create iperf test struct");
        }
        iperf_sys::iperf_defaults(test);
        iperf_sys::iperf_set_verbose(test, 2);
        iperf_sys::iperf_set_test_role(test, 'c' as i8);
        iperf_sys::iperf_set_test_server_hostname(
            test,
            std::ffi::CString::new("localhost").unwrap().into_raw(),
        );
        iperf_sys::iperf_set_test_server_port(test, 5201);
        let result = dbg!(iperf_sys::iperf_run_client(test));
        if result < 0 {
            panic!("Failed to run iperf client");
        }
        iperf_free_test(test);
    }
}
