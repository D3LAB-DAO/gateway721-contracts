# Gateway721 Contracts

<!-- TBD -->

---

# Quick Start

## Deploy

```bash
$ archway contracts build

$ archway contracts store gateway721

$ archway contracts instantiate gateway721 --args '{
  "name": "Gateway 721",
  "symbol": "GW721"
}'
```

## Metadata & Premiums

```bash
$ archway contracts metadata gateway721 --owner-address "archway1dqqfypr9a98czeh23a64eh6a0y7cqhycrzsm6a" --rewards-address "archway1dqqfypr9a98czeh23a64eh6a0y7cqhycrzsm6a"

# archway contracts premium gateway721 --premium-fee "1000000000000000000aconst"
```

## Execute

```bash
$ archway contracts execute gateway721 --args '{
  "mint": {
    "token_id": "0",
    "owner": "archway1dqqfypr9a98czeh23a64eh6a0y7cqhycrzsm6a",
    "extension": {
        "code": "function addNumbers(params) { const { a, b } = params; return a + b; } mainFunction = addNumbers;"
    }
  }
}'

$ archway contracts execute gateway721 --args '{
  "mint": {
    "token_id": "1",
    "owner": "archway1dqqfypr9a98czeh23a64eh6a0y7cqhycrzsm6a",
    "extension": {
        "code": "function calculateCircleArea(params) { const { radius } = params; const area = Math.PI * Math.pow(radius, 2); return area; } mainFunction = calculateCircleArea;"
    }
  }
}'

$ archway contracts execute gateway721 --args '{
  "request": {
    "token_id": "0",
    "input": "{ \"a\": 5, \"b\": 3 }"
  }
}'

$ archway contracts execute gateway721 --args '{
  "response": {
    "token_id": "0",
    "task_id": "0",
    "output": "8"
  }
}'

$ archway contracts execute gateway721 --args '{
  "update": {
    "token_id": "1",
    "title": "Circle Area Calculation",
    "description": "Calculates the area of a circle given its radius."
  }
}'
```

## Query

```bash
$ archway contracts query smart gateway721 --args '{
  "nft_info": {
    "token_id": "0"
  }
}'

$ archway contracts query smart gateway721 --args '{
  "remains": {
    "token_id": "0"
  }
}'
```
