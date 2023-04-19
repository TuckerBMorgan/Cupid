FROM rust

COPY target/release/cupid.exe /bin/cupid.exe

CMD ["\bin\cupid.exe"]