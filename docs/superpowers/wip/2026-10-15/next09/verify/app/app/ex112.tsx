<div
  className={`
    transition-all duration-300 ease-out
    ${
      isVisible
        ? "translate-y-0 opacity-100"
        : "translate-y-2 opacity-0"
    }
  `}
>
  {displayChildren}
</div>
