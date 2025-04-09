# Build for the web

1.
```console
$ npm install
```

2.
```console
$ npx wasm-pack build ".." --target web --out-name web --out-dir web/pkg
```

3.
```console
$ npm run serve
```

or

```console
$ npm run build
$ (cd dist && python3 -m http.server 8080)
```

4.
Open `http://localhost:8080` in a browser
