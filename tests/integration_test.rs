// Tests require an SDR to run
extern crate rtlsdr_rs;

#[test]
fn cancel() {
    let (mut ctl, mut reader) = rtlsdr_rs::open(0).unwrap();

    let handle = std::thread::spawn(move || {
        let mut count = 0;
        reader
            .read_async(4, 32768, |_| {
                count += 1;
            })
            .unwrap();
        count
    });

    ctl.cancel_async_read().unwrap();
    let count = handle.join().unwrap();
    println!("{}", count);
    assert!(false);
}
