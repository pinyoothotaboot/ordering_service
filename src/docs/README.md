## [1] - Create new order

```http
POST /api/v1/order/
```

## Request
```json
{
    "items" : [ 
        {
            "product_id" : "1234",
            "amount" : 16
        }
    ],
    "customer_id" : "1234-4321",
    "user_id" : "123456789"
}
```

## Responses

```json
{
    "code": 201,
    "success": true,
    "payload": {
        "message": "Create new order successfully",
        "data": [
            {
                "order_id": "62ad44ccd14c2fa4b0b79824"
            }
        ]
    }
}
```

## [2] - Update changed order

```http
PUT /api/v1/order/{order_id}/
```

## Request
```json
{
    "items" : [ 
        {
            "product_id" : "1234",
            "amount" : 5
        },
        {
            "product_id" : "4321",
            "amount" : 3
        }
    ],
    "customer_id" : "1234-4321",
    "user_id" : "123456789"
}
```

## Responses

```json
{
    "code": 200,
    "success": true,
    "payload": {
        "message": "Update order successfully",
        "data": [
            {
                "order_id": "62ad44ccd14c2fa4b0b79824"
            }
        ]
    }
}
```

## [3] - Confirm changed order

```http
PATCH /api/v1/order/{order_id}/
```

## Responses

```json
{
    "code": 200,
    "success": true,
    "payload": {
        "message": "Confirm order successfully",
        "data": [
            {
                "order_id": "62ad44ccd14c2fa4b0b79824"
            }
        ]
    }
}
```

## [4] - Cancel changed order

```http
DELETE /api/v1/order/{order_id}/
```

## Responses

```json
{
    "code": 204,
    "success": true,
    "payload": {
        "message": "Cancel order successfully",
        "data": [
            {
                "order_id": "62adc4a545a6695dc2452092"
            }
        ]
    }
}
```

## Status Codes

| Status Code | Description |
| :--- | :--- |
| 200 | `OK` |
| 201 | `CREATED` |
| 400 | `BAD REQUEST` |
| 404 | `NOT FOUND` |
| 500 | `INTERNAL SERVER ERROR` |