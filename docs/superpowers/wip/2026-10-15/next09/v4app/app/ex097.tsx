<div
  role="dialog"
  aria-modal="true"
  aria-labelledby="dialog-title"
  aria-describedby="dialog-description"
>
  <div className="... bg-black/50 ..." onClick={onCancel} aria-hidden="true" />
  <div ref={dialogRef} className="relative ...">
    <h2 id="dialog-title">{title}</h2>
    <p id="dialog-description">{message}</p>
    {/* ... ボタン ... */}
  </div>
</div>
