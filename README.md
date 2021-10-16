## Testing

```shell
cargo build
setcap CAP_NET_ADMIN=+eip ./target/debug/nflog_source
./target/debug/nflog_source
```

Iptables
```shell
iptables -N test_nflog
iptables -A INPUT -d 127.0.0.0/8 -p tcp --dport 3000 -j test_nflog
iptables -t mangle -A PREROUTING -d 127.0.0.0.8 -p tcp --dport 3000 -j CONNMARK --set-mark 1
iptables -A test_nflog -j CONNMARK --restore-mark
iptables -A test_nflog -j NFLOG --nflog-group 10 --nflog-prefix "Test nflog"
```

Start listening server:
```shell
nc -k -l 0.0.0.0 3000
```

Start client:
```shell
nc 127.0.0.1 3000
```