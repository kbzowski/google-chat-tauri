import { emit } from '@tauri-apps/api/event';

export type LogLevel = 'trace' | 'debug' | 'info' | 'warn' | 'error';

export const EVENT_LOG = 'injection-log';

export interface LogEntry {
  level: LogLevel;
  target: string;
  message: string;
  fields: Record<string, unknown>;
  timestamp: string;
}

export function makeEntry(
  level: LogLevel,
  target: string,
  message: string,
  fields: Record<string, unknown>,
): LogEntry {
  return {
    level,
    target,
    message,
    fields,
    timestamp: new Date().toISOString(),
  };
}

async function send(entry: LogEntry): Promise<void> {
  try {
    await emit(EVENT_LOG, entry);
  } catch {
    const method = entry.level === 'trace' ? 'debug' : entry.level;
    console[method](`[${entry.target}] ${entry.message}`, entry.fields);
  }
}

export const logger = (target: string) => ({
  trace: (msg: string, fields: Record<string, unknown> = {}) =>
    send(makeEntry('trace', target, msg, fields)),
  debug: (msg: string, fields: Record<string, unknown> = {}) =>
    send(makeEntry('debug', target, msg, fields)),
  info: (msg: string, fields: Record<string, unknown> = {}) =>
    send(makeEntry('info', target, msg, fields)),
  warn: (msg: string, fields: Record<string, unknown> = {}) =>
    send(makeEntry('warn', target, msg, fields)),
  error: (msg: string, fields: Record<string, unknown> = {}) =>
    send(makeEntry('error', target, msg, fields)),
});
