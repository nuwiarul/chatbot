declare module 'isomorphic-dompurify' {
  const DOMPurify: {
    sanitize: (dirty: string) => string;
  };
  export default DOMPurify;
}

