#ifndef RUST_PORT_H
#define RUST_PORT_H

class rust_port : public kernel_owned<rust_port>, public rust_cond {
public:
    RUST_REFCOUNTED(rust_port);

    rust_port_id id;

    rust_kernel *kernel;
    rust_task *task;
    rust_chan *remote_chan;
    size_t unit_sz;
    ptr_vec<rust_token> writers;
    ptr_vec<rust_chan> chans;

    lock_and_signal lock;

    rust_port(rust_task *task, size_t unit_sz);
    ~rust_port();
    void log_state();
    bool receive(void *dptr);
};

//
// Local Variables:
// mode: C++
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// compile-command: "make -k -C $RBUILD 2>&1 | sed -e 's/\\/x\\//x:\\//g'";
// End:
//

#endif /* RUST_PORT_H */
