FROM golang:1.21

WORKDIR /app

COPY . .

RUN go mod download
RUN CGO_ENABLED=0 GOOS=linux go build -o main
ENV GIN_MODE=release

EXPOSE 80

CMD ["./main"]