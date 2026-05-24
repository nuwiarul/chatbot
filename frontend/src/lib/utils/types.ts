export type Role = 'system' | 'user' | 'assistant';

export type ChatMessage = {
  role: Role;
  content: string;
};

export type ChatRequest = {
  messages: ChatMessage[];
};

export type ChatResponse = {
  message: ChatMessage;
};

export type ChatStreamEvent =
  | { type: 'delta'; data: string }
  | { type: 'done'; data: '[DONE]' }
  | { type: 'keep-alive'; data: string };
