<script lang="ts">
  import DOMPurify from 'isomorphic-dompurify';
  import { marked } from 'marked';

  export let text: string;

  const renderer = new marked.Renderer();
  renderer.blockquote = ({ tokens }) => {
    const inner = marked.Parser.parse(tokens);
    return `<blockquote class="border-l-4 border-gray-300 pl-3 italic">${inner}</blockquote>`;
  };
  renderer.code = ({ text: code, lang }) => {
    const language = lang ? ` data-lang="${escapeAttr(lang)}"` : '';
    return `<pre class="overflow-auto rounded bg-gray-950 p-3 text-gray-100"><code${language}>${escapeHtml(
      code
    )}</code></pre>`;
  };

  marked.setOptions({
    gfm: true,
    breaks: true,
    renderer
  });

  function escapeHtml(value: string): string {
    return value
      .replaceAll('&', '&amp;')
      .replaceAll('<', '&lt;')
      .replaceAll('>', '&gt;')
      .replaceAll('"', '&quot;')
      .replaceAll("'", '&#039;');
  }

  function escapeAttr(value: string): string {
    return escapeHtml(value).replaceAll('`', '&#096;');
  }

  function normalizeMarkdown(input: string): string {
    // Heuristics to make model output render nicer when fences are malformed.
    let s = input;

    // Ensure newline after opening fence (```lang<no newline>)
    s = s.replace(/```([a-zA-Z0-9_-]+)([^\r\n])/g, '```$1\n$2');
    s = s.replace(/```([^\r\n])/g, '```\n$1');

    // Ensure newline before closing fence when glued to previous token
    s = s.replace(/([^\r\n])```/g, '$1\n```');

    // If fences are unbalanced, close at the end
    const fenceCount = (s.match(/```/g) ?? []).length;
    if (fenceCount % 2 === 1) s += '\n```';

    return s;
  }

  $: html = DOMPurify.sanitize(marked.parse(normalizeMarkdown(text)) as string);
</script>

<div class="prose prose-sm max-w-none prose-pre:my-2 prose-p:my-2 prose-ul:my-2 prose-ol:my-2">
  {@html html}
</div>
