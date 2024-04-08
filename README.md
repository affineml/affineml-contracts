### devnet addr

```
Program Id: GD8gkSYT3nK3Y5kfNVZBXET98U6PUUrP65eMec7m1az8
```

### build

```
anchor build
```

### test

```
anchor test
```

### deploy

```
anchor deploy -p affineml  --provider.wallet id.json
```

# python client

### install python env (requires Python >= 3.9)

```
python3 -m venv venv
source venv/bin/activate
pip install anchorpy

```

### Client Generator

```
anchorpy client-gen target/idl/affineml.json ./python/python_client
```
