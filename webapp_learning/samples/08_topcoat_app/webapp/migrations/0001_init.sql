-- 依頼・承認アプリの表（06 の Django 版の models.py と同じ中身を、SQL で書いた物）
-- アプリが起動したときに sqlx が流す。1 度流したファイルは、あとから書きかえない（直すときは 0002_... を足す）

-- 社員（ログインする人）
CREATE TABLE employees (
    id            BIGSERIAL PRIMARY KEY,
    employee_no   TEXT    NOT NULL UNIQUE,                 -- 従業員番号（ログインに使う）
    name          TEXT    NOT NULL,
    dept          TEXT    NOT NULL DEFAULT '',
    job_title     TEXT    NOT NULL DEFAULT '',
    email         TEXT    NOT NULL,
    role          TEXT    NOT NULL DEFAULT 'member'
                  CHECK (role IN ('member', 'worker', 'admin')),
    password_hash TEXT,                                    -- NULL の人は、ログインできない
    is_active     BOOLEAN NOT NULL DEFAULT TRUE
);

-- ログイン中の記録。クッキーの文字列そのものではなく、その指紋（SHA-256）を持つ
CREATE TABLE sessions (
    token_hash  BYTEA       PRIMARY KEY,
    employee_id BIGINT      NOT NULL REFERENCES employees (id) ON DELETE CASCADE,
    expires_at  TIMESTAMPTZ NOT NULL
);

-- 依頼 1 件
CREATE TABLE requests (
    id              BIGSERIAL   PRIMARY KEY,
    title           TEXT        NOT NULL CHECK (char_length(title) BETWEEN 1 AND 100),
    detail          TEXT        NOT NULL DEFAULT '',
    due_date        DATE        NOT NULL,
    status          TEXT        NOT NULL DEFAULT 'circulating'
                    CHECK (status IN ('circulating', 'returned', 'pulled_back', 'approved',
                                      'in_progress', 'done', 'withdrawn')),
    requester_id    BIGINT      NOT NULL REFERENCES employees (id),
    assignee_id     BIGINT      REFERENCES employees (id),
    drawing         JSONB,                                 -- 図面（無ければ NULL）
    return_comments TEXT        NOT NULL DEFAULT '',       -- 差し戻すたびに追記する
    round           SMALLINT    NOT NULL DEFAULT 1,        -- 出し直すたびに 1 増える
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX requests_requester_id ON requests (requester_id);

-- 回覧の 1 人分。承認の URL の指紋も、ここに持つ
CREATE TABLE steps (
    id               BIGSERIAL   PRIMARY KEY,
    request_id       BIGINT      NOT NULL REFERENCES requests (id) ON DELETE CASCADE,
    round            SMALLINT    NOT NULL,
    step_order       SMALLINT    NOT NULL,                 -- 何番目か（order は SQL の言葉なので避ける）
    approver_id      BIGINT      NOT NULL REFERENCES employees (id),
    decision         TEXT        NOT NULL DEFAULT 'waiting'
                     CHECK (decision IN ('waiting', 'approved', 'returned', 'cancelled')),
    decided_at       TIMESTAMPTZ,
    token_hash       TEXT,                                 -- URL の文字列の指紋（16 進）
    token_expires_at TIMESTAMPTZ,
    sent_at          TIMESTAMPTZ,
    UNIQUE (request_id, round, step_order)
);
CREATE INDEX steps_token_hash ON steps (token_hash);

-- 添付ファイル。中身はフォルダ（FILE_DIR）に置き、ここには名前と大きさだけを持つ
CREATE TABLE attachments (
    id            BIGSERIAL   PRIMARY KEY,
    request_id    BIGINT      NOT NULL REFERENCES requests (id) ON DELETE CASCADE,
    stored_name   TEXT        NOT NULL,                    -- フォルダに置いた名前（アプリが決める）
    original_name TEXT        NOT NULL,                    -- 送られてきたときの名前（表示とダウンロードに使う）
    content_type  TEXT        NOT NULL,
    size          INTEGER     NOT NULL,
    uploaded_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- だれが、いつ、何をしたかの記録（あとから変えない）
CREATE TABLE histories (
    id         BIGSERIAL   PRIMARY KEY,
    request_id BIGINT      NOT NULL REFERENCES requests (id) ON DELETE CASCADE,
    at         TIMESTAMPTZ NOT NULL DEFAULT now(),
    actor_id   BIGINT      REFERENCES employees (id),      -- 毎朝の処理など、人でないときは NULL
    action     TEXT        NOT NULL,
    note       TEXT        NOT NULL DEFAULT ''
);

-- 毎朝の処理を、その日に済ませたかの記録（同じ日に 2 回送らないため）
CREATE TABLE job_runs (
    job         TEXT        NOT NULL,
    run_date    DATE        NOT NULL,
    started_at  TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (job, run_date)
);
