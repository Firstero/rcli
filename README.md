# Geektime Rust 语言训练营

## Rcli 开发

### V1-7 rcli csv 添加 csv 转换

    - ```rcli csv --input in.csv --output out.json --format json```
    - ```rcli csv --input in.csv --output out.yaml --format yaml```
    - ```rcli csv --header --delimiter , --input in.csv --output out.yaml --format yaml```

### V1-8 rcli genpass

    - ```rcli genpass -l 32 --no-lower --no-lower --no-symbol --no-number```

### V1-9 rcli base64

    - ```rcli base64 encode --format nopadding/standard/urlsafe --input textfile```
    - ```rcli base64 decode --format nopadding/standard/urlsafe --input textfile```

### V1-10/11 rcli text 加密解密

    - ```rcli text sign --format blake3 --key keyfile --input textfile```
    - ```rcli text verify --format blake3 --key keyfile --input textfile --sig signature```
    - ```rcli text generate --format blake3 --output keyfile```
    - ```rcli text encrypt --key keyfile --input textfile --nonce noncefile```
    - ```rcli text decrypt --key keyfile --input textfile --nonce noncefile```

### V1-12/13 rcli http serve(default dir is current dir, default port is 8080)

    - ```rcli http serve```
    - ```rcli http serve --dir /tmp --port 8080```

### V1-14 重构 CLI

### V1-15 作业
