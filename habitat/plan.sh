pkg_name=container_sched_frontend
pkg_origin=thom
pkg_version="0.1.0"
pkg_build_deps=(core/rust)
pkg_deps=(core/glibc core/gcc core/gcc-libs)
pkg_bin_dirs=(bin)
bin="container_sched_frontend"
pkg_binds=([backend]="out")

do_build() {
  cargo build
}
do_install() {
  install -v -D "$PLAN_CONTEXT/../target/debug/$bin" \
    "$pkg_prefix/bin/$bin"
}
pkg_svc_run="$bin"
