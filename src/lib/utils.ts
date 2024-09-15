export function urlEncode(str: string) {
  return btoa(str).replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
}

export function urlDecode(str: string) {
  str = str.replace(/-/g, '+').replace(/_/g, '/');

  switch (str.length % 4) {
    case 2:
      str += '==';
      break;
    case 3:
      str += '=';
      break;
  }

  return atob(str);
}

export function toSentenceCase(str: string): string {
  if (!str) return str;
  return str.charAt(0).toUpperCase() + str.slice(1).toLowerCase();
}

export const isInvokeErr = (data: any): data is InvokeErr => {
  return 'err' in data && typeof data.err === 'string';
}
