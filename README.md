# Build App
```shell
docker build -t cryptify-builder .
```

# Copy to Root
```shell
docker run --rm -v $(pwd):/output cryptify-builder cp /app/cryptify /output
```