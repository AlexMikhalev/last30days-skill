import fs from 'fs/promises';
import path from 'path';

default function slugify(s){
  return s
    .toLowerCase()
    .replace(/\s+/g,'-')
    .replace(/[^a-z0-9\-]+/g,'')
    .replace(/\-+/g,'-')
    .replace(/^\-|-$/g,'')
    .slice(0,120) || 'untitled';
}
