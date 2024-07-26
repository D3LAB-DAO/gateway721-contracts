# Gateway721 Contracts

<!-- TBD -->

<!--

AI-driven title and description

-->

<!-- 

// Callee
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OtherContractExecuteMsg {
    ReceiveOutput { output: String },
}

-->

<!--

- AI 상호작용으로 대화형으로 js 배포 및 사용
-- AI 상호작용으로 자연어로부터 코드 생성
-- AI 상호작용으로 js 코드 포맷에 맞게 수정

- CLI (archway-cli 처럼 혹은 통합)

-->

## Interface

```
pub enum OtherContractExecuteMsg {
    ReceiveOutput { output: String },
}
```

This interface allows for the transmission of JavaScript code execution results to another contract.
When minting, it is mandatory to set the `destination` field (address of the other contract).

---

# Deploy

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
$ archway contracts metadata gateway721 --owner-address "archway1r0cmlns8ta3hckzlpalennsxxv5erfgnz3qq0s" --rewards-address "archway1r0cmlns8ta3hckzlpalennsxxv5erfgnz3qq0s"

# archway contracts premium gateway721 --premium-fee "1000000000000000000aconst"
```

# Examples

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
  "request": {
    "token_id": "0",
    "input": "{ \"a\": 1, \"b\": 2 }"
  }
}'

$ archway contracts execute gateway721 --args '{
  "request": {
    "token_id": "1",
    "input": "{ \"radius\": 10 }"
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

$ archway contracts query smart gateway721 --args '{"incomplete_projects": {}}'
$ archway contracts query smart gateway721 --args '{"num_tokens": {}}'
```
