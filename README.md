# testeksempel med jaeger og rust

```sh
docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p14268:14268 jaegertracing/all-in-one:latest
```

enkelt eksempel for å kjøre i gang opentelemetry-tracing med jaeger og logging til konsoll
