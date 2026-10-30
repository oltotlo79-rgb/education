"""第8巻 UNIT 06 のわざと間違えた版（と、直した版）の置き換え表。
src/routes.rs の build_router に、OriginPolicy の設定を1行足す。
"""

IMPORT = ("        Body, Next, Router, Slot,\n", "        Body, Next, OriginPolicy, Router, Slot,\n")


def policy(comment, expr):
    return [
        IMPORT,
        (
            "        .app_context(state)\n        .build()",
            f"        .app_context(state)\n        // {comment}\n        .origin_policy({expr})\n        .build()",
        ),
    ]


DISABLE = policy("わざと間違えた版: SameSite=Lax があるから要らない、と考えて外した", "OriginPolicy::dangerous_disable()")
SLASH = policy("わざと間違えた版: 決済サービスからの POST を信じる（最後に / を付けてしまった）", 'OriginPolicy::new().trust_origins(["https://pay.example/"])')
OK = policy("直した版: 決済サービスからの POST を信じる（/ を付けない）", 'OriginPolicy::new().trust_origins(["https://pay.example"])')
