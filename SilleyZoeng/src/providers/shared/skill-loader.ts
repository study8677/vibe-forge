import fs from 'node:fs';
import path from 'node:path';
import matter from 'gray-matter';
import type { SkillMeta } from './types.js';

const DEFAULT_SKILLS_DIR = path.join(
  process.env.HOME || '~',
  '.claude',
  'skills',
);

export function getSkillsDir(): string {
  const dir = process.env.SKILLS_DIR || DEFAULT_SKILLS_DIR;
  return dir.replace(/^~/, process.env.HOME || '~');
}

export function loadSkill(skillName: string): SkillMeta {
  const skillDir = path.join(getSkillsDir(), skillName);
  const skillPath = path.join(skillDir, 'SKILL.md');

  if (!fs.existsSync(skillPath)) {
    throw new Error(`Skill not found: ${skillPath}`);
  }

  const raw = fs.readFileSync(skillPath, 'utf-8');
  const { data, content } = matter(raw);

  return {
    name: data.name || skillName,
    slug: data.slug || skillName,
    description: typeof data.description === 'string'
      ? data.description
      : JSON.stringify(data.description || ''),
    content: content.trim(),
  };
}

export function loadAllSkills(): SkillMeta[] {
  const dir = getSkillsDir();
  if (!fs.existsSync(dir)) {
    throw new Error(`Skills directory not found: ${dir}`);
  }

  const entries = fs.readdirSync(dir, { withFileTypes: true });
  const skills: SkillMeta[] = [];

  for (const entry of entries) {
    if (!entry.isDirectory()) continue;
    const skillPath = path.join(dir, entry.name, 'SKILL.md');
    if (!fs.existsSync(skillPath)) continue;

    try {
      skills.push(loadSkill(entry.name));
    } catch {
      // Skip skills that fail to load
    }
  }

  return skills;
}

export function listSkillNames(): string[] {
  const dir = getSkillsDir();
  if (!fs.existsSync(dir)) return [];

  return fs.readdirSync(dir, { withFileTypes: true })
    .filter(e => e.isDirectory() && fs.existsSync(path.join(dir, e.name, 'SKILL.md')))
    .map(e => e.name);
}

// Self-test when run directly
if (process.argv[1] && process.argv[1].includes('skill-loader')) {
  const skills = loadAllSkills();
  console.log(`Loaded ${skills.length} skills:`);
  for (const s of skills) {
    console.log(`  - ${s.name} (${s.slug}): ${s.description.slice(0, 80)}...`);
  }
}
