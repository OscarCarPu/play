FROM golang:1.25.7-alpine AS builder

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY . .

RUN CGO_ENABLED=0 go build -ldflags="-w -s" -o svr cmd/httpserver/*.go

FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/svr .
EXPOSE 8080
CMD ["./svr"]
