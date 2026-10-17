const renderIcon = () => {
  switch (icon) {
    case "book":
      return ( <svg ...>{/* 本のアイコン */}</svg> );
    case "search":
      return ( <svg ...>{/* 虫眼鏡のアイコン */}</svg> );
    case "error":
      return ( <svg ...>{/* エラーのアイコン */}</svg> );
  }
};
