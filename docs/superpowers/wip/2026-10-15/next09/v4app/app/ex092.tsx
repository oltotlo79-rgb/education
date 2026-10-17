{isMobileMenuOpen && (
  <nav id="mobile-menu" className="border-t ... md:hidden" role="navigation" aria-label="モバイルナビゲーション">
    <ul className="flex flex-col gap-1" role="list">
      {navItems.map((item) => (
        // ... 各リンク。onClick で setIsMobileMenuOpen(false)
      ))}
    </ul>
  </nav>
)}
