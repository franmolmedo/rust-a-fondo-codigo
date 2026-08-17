let old_style = || async {
    do_work().await
};

let first_class = async || {
    do_work().await
};
