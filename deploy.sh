#!/bin/sh

cargo build --release && \
scp target/armv7-unknown-linux-gnueabihf/release/remacle remarkable-wifi:remacle && \
(ssh remarkable-wifi /home/root/remacle/run.sh ; \
ssh remarkable-wifi killall remacle)
