# Sled-RPC

An RPC server to `set/get` values through __sled__.

## Set

Set data in database directly: 

```json
{
	"id": "1",
	"method": "x",
	"jsonrpc": "2.0",
	"params": {
		"db": "test_db",
		"key": "test_key",
		"value": "test_value"
	}
}
```

Set data in tree:

```json
{
	"id": "1",
	"method": "x",
	"jsonrpc": "2.0",
	"params": {
		"db": "test_db",
		"tree": "test_tree",
		"key": "test_key",
		"value": "test_value"
	}
}
```

## Get

```json
{
	"id": "1",
	"method": "x",
	"jsonrpc": "2.0",
	"params": {
		"db": "test_db",
		"tree": "test_tree",
		"key": "test_key",
	}
}
```

## Batch

```json
{
	"id": "1",
	"method": "x",
	"jsonrpc": "2.0",
	"params": {
		"db": "test_db",
		"tree": "test_tree",
		"batch": "true"
	}
}
```
