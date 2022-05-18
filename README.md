# meas

Measure duration of execution of a command. Kind of like `time(1)`.

```
$ meas -- ping localhost -c5
00:00.004 │ PING localhost (127.0.0.1): 56 data bytes
00:00.005 │ 64 bytes from 127.0.0.1: icmp_seq=0 ttl=64 time=0.053 ms
00:01.006 │ 64 bytes from 127.0.0.1: icmp_seq=1 ttl=64 time=0.177 ms
00:02.008 │ 64 bytes from 127.0.0.1: icmp_seq=2 ttl=64 time=0.108 ms
00:03.011 │ 64 bytes from 127.0.0.1: icmp_seq=3 ttl=64 time=0.092 ms
00:04.014 │ 64 bytes from 127.0.0.1: icmp_seq=4 ttl=64 time=0.122 ms
00:04.014 │
00:04.014 │ --- localhost ping statistics ---
00:04.014 │ 5 packets transmitted, 5 packets received, 0.0% packet loss
00:04.014 │ round-trip min/avg/max/stddev = 0.053/0.110/0.177/0.041 ms
command completed in 4s 15ms
```

## install

```sh
$ cargo install meas
```

## one of these days (todo)

- add parameters that control from where to start and end measuring command run
  time
- run a command n times and write a report
