#
# sample dockerfile for jsonrpc
#

FROM ubuntu:18.04

# use a mini linux : alpine
#FROM alpine:latest
ADD target/release/jsonrpc /bin/jsonrpc

ENV RUST_BACKTRACE=1
CMD /bin/jsonrpc
