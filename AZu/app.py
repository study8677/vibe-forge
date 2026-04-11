"""
╔══════════════════════════════════════════════════════════════════╗
║   CYBERPUNK DREAM GENERATOR v2.0.77                            ║
║   AI Dream Art × Synth Music × Code Matrix                     ║
║   Neon-Drenched Neural Playground for Hugging Face Spaces       ║
╚══════════════════════════════════════════════════════════════════╝
"""

import gradio as gr
import numpy as np
from PIL import Image, ImageDraw, ImageFilter, ImageChops, ImageEnhance
import math
import random
import tempfile
import wave
import os
import colorsys
import hashlib
import time

# ═══════════════════════════════════════════════════════════════
#  CYBERPUNK COLOR PALETTES
# ═══════════════════════════════════════════════════════════════

PALETTES = {
    "🔮 Neon Noir": {
        "primary": (255, 0, 255),
        "secondary": (180, 0, 255),
        "accent": (255, 0, 128),
        "bg_top": (5, 0, 30),
        "bg_bottom": (30, 0, 50),
        "glow": (255, 50, 255),
    },
    "🌊 Cyber Ocean": {
        "primary": (0, 255, 255),
        "secondary": (0, 128, 255),
        "accent": (0, 255, 180),
        "bg_top": (0, 5, 30),
        "bg_bottom": (0, 20, 50),
        "glow": (50, 255, 255),
    },
    "🔥 Digital Inferno": {
        "primary": (255, 100, 0),
        "secondary": (255, 50, 0),
        "accent": (255, 200, 0),
        "bg_top": (20, 5, 0),
        "bg_bottom": (40, 10, 0),
        "glow": (255, 150, 50),
    },
    "🌿 Matrix Green": {
        "primary": (0, 255, 65),
        "secondary": (0, 200, 50),
        "accent": (100, 255, 0),
        "bg_top": (0, 10, 0),
        "bg_bottom": (0, 25, 10),
        "glow": (50, 255, 100),
    },
    "⚡ Electric Storm": {
        "primary": (150, 100, 255),
        "secondary": (100, 150, 255),
        "accent": (200, 200, 255),
        "bg_top": (10, 5, 30),
        "bg_bottom": (20, 15, 50),
        "glow": (180, 150, 255),
    },
    "🌈 Rainbow Glitch": {
        "primary": (255, 0, 255),
        "secondary": (0, 255, 255),
        "accent": (255, 255, 0),
        "bg_top": (10, 0, 15),
        "bg_bottom": (15, 5, 25),
        "glow": (255, 128, 255),
    },
}

# Musical notes (Hz)
NOTE_FREQS = {
    "C": [32.70, 65.41, 130.81, 261.63, 523.25, 1046.50],
    "C#": [34.65, 69.30, 138.59, 277.18, 554.37, 1108.73],
    "D": [36.71, 73.42, 146.83, 293.66, 587.33, 1174.66],
    "D#": [38.89, 77.78, 155.56, 311.13, 622.25, 1244.51],
    "E": [41.20, 82.41, 164.81, 329.63, 659.26, 1318.51],
    "F": [43.65, 87.31, 174.61, 349.23, 698.46, 1396.91],
    "F#": [46.25, 92.50, 185.00, 369.99, 739.99, 1479.98],
    "G": [49.00, 98.00, 196.00, 392.00, 783.99, 1567.98],
    "G#": [51.91, 103.83, 207.65, 415.30, 830.61, 1661.22],
    "A": [55.00, 110.00, 220.00, 440.00, 880.00, 1760.00],
    "A#": [58.27, 116.54, 233.08, 466.16, 932.33, 1864.66],
    "B": [61.74, 123.47, 246.94, 493.88, 987.77, 1975.53],
}

CHORD_PATTERNS = {
    "minor": [0, 3, 7],
    "major": [0, 4, 7],
    "minor7": [0, 3, 7, 10],
    "major7": [0, 4, 7, 11],
    "sus4": [0, 5, 7],
    "dim": [0, 3, 6],
}

SCALE_NOTES = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"]

# ═══════════════════════════════════════════════════════════════
#  CYBERPUNK CSS — THE VISUAL NUCLEAR CORE
# ═══════════════════════════════════════════════════════════════

CYBERPUNK_CSS = """
@import url('https://fonts.googleapis.com/css2?family=Orbitron:wght@400;500;600;700;800;900&family=Share+Tech+Mono&family=Rajdhani:wght@300;400;500;600;700&family=Audiowide&display=swap');

:root {
    --neon-pink: #ff00ff;
    --neon-cyan: #00ffff;
    --neon-green: #00ff41;
    --neon-orange: #ff6b35;
    --neon-yellow: #ffd700;
    --neon-purple: #bf00ff;
    --neon-red: #ff0040;
    --dark-void: #050508;
    --dark-bg: #0a0a12;
    --dark-card: #0d0d1a;
    --dark-surface: #111125;
    --dark-border: #1a1a3e;
    --dark-hover: #1e1e42;
    --text-primary: #e0e0ff;
    --text-secondary: #8888bb;
    --text-dim: #555580;
}

/* ═══ GLOBAL RESET & BASE ═══ */
* { box-sizing: border-box; }

body, .gradio-container {
    background: var(--dark-void) !important;
    color: var(--text-primary) !important;
    font-family: 'Rajdhani', 'Share Tech Mono', monospace !important;
    min-height: 100vh;
}

.gradio-container {
    max-width: 1400px !important;
    margin: 0 auto !important;
    position: relative;
    overflow-x: hidden;
}

/* ═══ ANIMATED BACKGROUND ═══ */
.gradio-container::before {
    content: '';
    position: fixed;
    top: 0; left: 0; right: 0; bottom: 0;
    background:
        linear-gradient(180deg,
            rgba(5,0,20,0.97) 0%,
            rgba(10,5,30,0.95) 30%,
            rgba(15,0,40,0.97) 60%,
            rgba(5,5,15,0.99) 100%
        ),
        repeating-linear-gradient(
            0deg,
            transparent,
            transparent 50px,
            rgba(255,0,255,0.015) 50px,
            rgba(255,0,255,0.015) 51px
        ),
        repeating-linear-gradient(
            90deg,
            transparent,
            transparent 50px,
            rgba(0,255,255,0.015) 50px,
            rgba(0,255,255,0.015) 51px
        );
    z-index: -2;
    animation: bgShift 20s ease-in-out infinite;
}

/* ═══ SCAN LINE OVERLAY ═══ */
.gradio-container::after {
    content: '';
    position: fixed;
    top: 0; left: 0; right: 0; bottom: 0;
    background: repeating-linear-gradient(
        0deg,
        transparent,
        transparent 2px,
        rgba(0, 0, 0, 0.08) 2px,
        rgba(0, 0, 0, 0.08) 4px
    );
    z-index: 9999;
    pointer-events: none;
    animation: scanMove 8s linear infinite;
}

/* ═══ HEADER ═══ */
.cyber-header {
    text-align: center;
    padding: 40px 20px 25px;
    position: relative;
    border-bottom: 1px solid rgba(255,0,255,0.15);
    margin-bottom: 10px;
    overflow: hidden;
}

.cyber-header::before {
    content: '';
    position: absolute;
    top: 0; left: 0; right: 0; bottom: 0;
    background: radial-gradient(ellipse at 50% 0%, rgba(255,0,255,0.08) 0%, transparent 60%),
                radial-gradient(ellipse at 50% 100%, rgba(0,255,255,0.05) 0%, transparent 60%);
    z-index: 0;
}

.cyber-header > * { position: relative; z-index: 1; }

.glitch-title {
    font-family: 'Orbitron', monospace !important;
    font-size: 3.2rem;
    font-weight: 900;
    letter-spacing: 6px;
    color: #ffffff;
    text-shadow:
        0 0 7px var(--neon-pink),
        0 0 15px var(--neon-pink),
        0 0 30px var(--neon-pink),
        0 0 50px rgba(255,0,255,0.5),
        0 0 80px rgba(255,0,255,0.3);
    margin: 0 0 8px 0;
    position: relative;
    display: inline-block;
    animation: textFlicker 4s infinite;
}

.glitch-title::before,
.glitch-title::after {
    content: attr(data-text);
    position: absolute;
    top: 0; left: 0;
    width: 100%; height: 100%;
    overflow: hidden;
}

.glitch-title::before {
    color: var(--neon-cyan);
    z-index: -1;
    animation: glitch1 3s infinite linear alternate-reverse;
    clip-path: polygon(0 0, 100% 0, 100% 40%, 0 40%);
}

.glitch-title::after {
    color: var(--neon-pink);
    z-index: -2;
    animation: glitch2 2s infinite linear alternate-reverse;
    clip-path: polygon(0 60%, 100% 60%, 100% 100%, 0 100%);
}

.cyber-subtitle {
    font-family: 'Share Tech Mono', monospace !important;
    font-size: 1.15rem;
    color: var(--neon-cyan);
    letter-spacing: 8px;
    text-shadow: 0 0 10px rgba(0,255,255,0.5);
    margin: 8px 0 18px 0;
    text-transform: uppercase;
}

.status-bar {
    display: flex;
    justify-content: center;
    gap: 20px;
    flex-wrap: wrap;
    font-family: 'Share Tech Mono', monospace;
    font-size: 0.78rem;
    color: var(--text-secondary);
    letter-spacing: 1px;
}

.status-item {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 4px 12px;
    background: rgba(255,0,255,0.05);
    border: 1px solid rgba(255,0,255,0.12);
    border-radius: 3px;
}

.status-dot {
    width: 6px; height: 6px;
    border-radius: 50%;
    display: inline-block;
}

.status-dot.online { background: var(--neon-green); box-shadow: 0 0 6px var(--neon-green); animation: dotPulse 2s infinite; }
.status-dot.active { background: var(--neon-cyan); box-shadow: 0 0 6px var(--neon-cyan); animation: dotPulse 2.5s infinite; }
.status-dot.ready { background: var(--neon-yellow); box-shadow: 0 0 6px var(--neon-yellow); animation: dotPulse 3s infinite; }
.status-dot.hot { background: var(--neon-pink); box-shadow: 0 0 6px var(--neon-pink); animation: dotPulse 1.5s infinite; }

/* ═══ TAB NAVIGATION ═══ */
.tabs > .tab-nav {
    background: transparent !important;
    border: none !important;
    padding: 0 !important;
    margin-bottom: 8px !important;
    display: flex !important;
    gap: 4px !important;
    justify-content: center !important;
}

.tabs > .tab-nav > button {
    background: rgba(13,13,26,0.8) !important;
    color: var(--text-secondary) !important;
    border: 1px solid var(--dark-border) !important;
    border-bottom: 2px solid transparent !important;
    border-radius: 4px 4px 0 0 !important;
    padding: 12px 24px !important;
    font-family: 'Orbitron', monospace !important;
    font-size: 0.85rem !important;
    font-weight: 600 !important;
    letter-spacing: 2px !important;
    transition: all 0.3s ease !important;
    position: relative;
    overflow: hidden;
    text-transform: uppercase !important;
}

.tabs > .tab-nav > button:hover {
    background: rgba(255,0,255,0.08) !important;
    color: var(--neon-pink) !important;
    border-color: rgba(255,0,255,0.3) !important;
    text-shadow: 0 0 8px rgba(255,0,255,0.5);
}

.tabs > .tab-nav > button.selected {
    background: rgba(255,0,255,0.1) !important;
    color: var(--neon-cyan) !important;
    border-color: var(--neon-pink) !important;
    border-bottom-color: var(--neon-cyan) !important;
    text-shadow: 0 0 10px rgba(0,255,255,0.6);
    box-shadow:
        0 0 15px rgba(255,0,255,0.15),
        inset 0 0 15px rgba(255,0,255,0.05);
}

/* ═══ BLOCKS & CARDS ═══ */
.block {
    background: var(--dark-card) !important;
    border: 1px solid var(--dark-border) !important;
    border-radius: 6px !important;
    box-shadow: 0 2px 15px rgba(0,0,0,0.4) !important;
    transition: all 0.3s ease !important;
}

.block:hover {
    border-color: rgba(255,0,255,0.2) !important;
    box-shadow: 0 2px 20px rgba(255,0,255,0.08) !important;
}

/* ═══ LABELS ═══ */
label, .label-wrap, .block > label span {
    color: var(--neon-cyan) !important;
    font-family: 'Orbitron', monospace !important;
    font-size: 0.75rem !important;
    font-weight: 600 !important;
    letter-spacing: 2px !important;
    text-transform: uppercase !important;
    text-shadow: 0 0 5px rgba(0,255,255,0.3);
}

/* ═══ INPUTS ═══ */
input[type="text"], input[type="number"], textarea, .wrap input {
    background: var(--dark-void) !important;
    color: var(--text-primary) !important;
    border: 1px solid var(--dark-border) !important;
    border-radius: 4px !important;
    font-family: 'Share Tech Mono', monospace !important;
    font-size: 0.95rem !important;
    transition: all 0.3s ease !important;
    padding: 10px 14px !important;
}

input[type="text"]:focus, input[type="number"]:focus, textarea:focus, .wrap input:focus {
    border-color: var(--neon-pink) !important;
    box-shadow: 0 0 12px rgba(255,0,255,0.2), inset 0 0 8px rgba(255,0,255,0.05) !important;
    outline: none !important;
}

textarea {
    min-height: 100px !important;
}

/* ═══ BUTTONS ═══ */
button.primary, button.lg.primary {
    background: linear-gradient(135deg, rgba(255,0,255,0.25), rgba(0,255,255,0.15)) !important;
    color: #ffffff !important;
    border: 1px solid var(--neon-pink) !important;
    border-radius: 4px !important;
    font-family: 'Orbitron', monospace !important;
    font-weight: 700 !important;
    font-size: 0.9rem !important;
    letter-spacing: 3px !important;
    text-transform: uppercase !important;
    padding: 14px 30px !important;
    cursor: pointer !important;
    position: relative;
    overflow: hidden;
    transition: all 0.3s ease !important;
    text-shadow: 0 0 8px rgba(255,0,255,0.5);
    box-shadow: 0 0 20px rgba(255,0,255,0.15), inset 0 0 20px rgba(255,0,255,0.05) !important;
}

button.primary:hover, button.lg.primary:hover {
    background: linear-gradient(135deg, rgba(255,0,255,0.4), rgba(0,255,255,0.25)) !important;
    box-shadow: 0 0 30px rgba(255,0,255,0.3), 0 0 60px rgba(255,0,255,0.1), inset 0 0 25px rgba(255,0,255,0.08) !important;
    transform: translateY(-1px);
}

button.primary::before {
    content: '';
    position: absolute;
    top: -50%; left: -50%;
    width: 200%; height: 200%;
    background: conic-gradient(transparent, rgba(255,0,255,0.1), transparent 30%);
    animation: borderSpin 4s linear infinite;
}

button.secondary {
    background: rgba(0,255,255,0.08) !important;
    color: var(--neon-cyan) !important;
    border: 1px solid rgba(0,255,255,0.25) !important;
    font-family: 'Share Tech Mono', monospace !important;
    letter-spacing: 1px !important;
    transition: all 0.3s ease !important;
}

button.secondary:hover {
    background: rgba(0,255,255,0.15) !important;
    box-shadow: 0 0 15px rgba(0,255,255,0.15) !important;
}

/* ═══ DROPDOWNS ═══ */
.wrap > .wrap, .dropdown-container, select,
div[data-testid="dropdown"] > div,
.secondary-wrap, .border-none {
    background: var(--dark-void) !important;
    color: var(--text-primary) !important;
    border: 1px solid var(--dark-border) !important;
    border-radius: 4px !important;
    font-family: 'Share Tech Mono', monospace !important;
}

/* ═══ SLIDERS ═══ */
input[type="range"] {
    accent-color: var(--neon-pink) !important;
}

.range-slider input[type="number"] {
    background: var(--dark-void) !important;
    color: var(--neon-cyan) !important;
    border: 1px solid var(--dark-border) !important;
    font-family: 'Share Tech Mono', monospace !important;
    width: 70px !important;
}

/* ═══ IMAGE OUTPUT ═══ */
.image-container, .image-frame, div[data-testid="image"] {
    border: 2px solid var(--dark-border) !important;
    border-radius: 6px !important;
    overflow: hidden;
    position: relative;
    box-shadow: 0 0 25px rgba(255,0,255,0.1) !important;
    transition: all 0.3s ease !important;
}

.image-container:hover, div[data-testid="image"]:hover {
    border-color: var(--neon-pink) !important;
    box-shadow: 0 0 35px rgba(255,0,255,0.2), 0 0 60px rgba(255,0,255,0.05) !important;
}

/* ═══ AUDIO PLAYER ═══ */
audio {
    width: 100% !important;
    filter: hue-rotate(280deg) saturate(1.5) !important;
}

/* ═══ CODE OUTPUT ═══ */
.code-wrap, pre, code {
    background: var(--dark-void) !important;
    color: var(--neon-green) !important;
    border: 1px solid var(--dark-border) !important;
    border-radius: 4px !important;
    font-family: 'Share Tech Mono', monospace !important;
    font-size: 0.88rem !important;
    text-shadow: 0 0 3px rgba(0,255,65,0.3);
}

/* ═══ ACCORDION ═══ */
.accordion {
    background: var(--dark-card) !important;
    border: 1px solid var(--dark-border) !important;
    border-radius: 6px !important;
}

/* ═══ CUSTOM SCROLLBAR ═══ */
::-webkit-scrollbar { width: 6px; height: 6px; }
::-webkit-scrollbar-track { background: var(--dark-void); }
::-webkit-scrollbar-thumb {
    background: linear-gradient(180deg, var(--neon-pink), var(--neon-purple));
    border-radius: 3px;
}
::-webkit-scrollbar-thumb:hover { background: var(--neon-pink); }

/* ═══ PROGRESS BAR ═══ */
.progress-bar {
    background: linear-gradient(90deg, var(--neon-pink), var(--neon-cyan), var(--neon-green)) !important;
    box-shadow: 0 0 15px rgba(255,0,255,0.4) !important;
    animation: progressGlow 2s ease-in-out infinite !important;
}

/* ═══ CYBER LOG ═══ */
.cyber-log textarea {
    background: var(--dark-void) !important;
    color: var(--neon-green) !important;
    font-family: 'Share Tech Mono', monospace !important;
    font-size: 0.82rem !important;
    text-shadow: 0 0 4px rgba(0,255,65,0.4);
    border: 1px solid rgba(0,255,65,0.15) !important;
    line-height: 1.6 !important;
}

/* ═══ SYSTEM CORE PANEL ═══ */
.system-panel {
    background: var(--dark-card);
    border: 1px solid var(--dark-border);
    border-radius: 8px;
    padding: 25px;
    margin: 10px 0;
    position: relative;
    overflow: hidden;
}

.system-panel::before {
    content: '';
    position: absolute;
    top: 0; left: 0; right: 0;
    height: 2px;
    background: linear-gradient(90deg, transparent, var(--neon-pink), var(--neon-cyan), var(--neon-green), transparent);
    animation: dataStream 3s linear infinite;
}

.system-ascii {
    font-family: 'Share Tech Mono', monospace;
    font-size: 0.65rem;
    color: var(--neon-cyan);
    text-shadow: 0 0 5px rgba(0,255,255,0.4);
    line-height: 1.2;
    text-align: center;
    white-space: pre;
    margin: 15px 0;
    opacity: 0.9;
}

.system-stats {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 15px;
    margin: 20px 0;
}

.stat-card {
    background: rgba(10,10,20,0.8);
    border: 1px solid var(--dark-border);
    border-radius: 6px;
    padding: 18px;
    transition: all 0.3s ease;
}

.stat-card:hover {
    border-color: var(--neon-pink);
    box-shadow: 0 0 20px rgba(255,0,255,0.1);
}

.stat-card h4 {
    font-family: 'Orbitron', monospace;
    font-size: 0.7rem;
    color: var(--neon-pink);
    letter-spacing: 2px;
    margin: 0 0 12px 0;
    text-transform: uppercase;
    text-shadow: 0 0 5px rgba(255,0,255,0.3);
}

.stat-line {
    font-family: 'Share Tech Mono', monospace;
    font-size: 0.8rem;
    color: var(--text-secondary);
    margin: 6px 0;
    display: flex;
    justify-content: space-between;
}

.stat-value { color: var(--neon-cyan); text-shadow: 0 0 4px rgba(0,255,255,0.3); }
.stat-value.green { color: var(--neon-green); text-shadow: 0 0 4px rgba(0,255,65,0.3); }
.stat-value.pink { color: var(--neon-pink); text-shadow: 0 0 4px rgba(255,0,255,0.3); }
.stat-value.yellow { color: var(--neon-yellow); text-shadow: 0 0 4px rgba(255,215,0,0.3); }

.progress-block {
    font-family: 'Share Tech Mono', monospace;
    font-size: 0.8rem;
    color: var(--neon-green);
    letter-spacing: 1px;
}

/* ═══ EXAMPLES ═══ */
.examples-row { margin-top: 8px !important; }
.examples-row button {
    background: rgba(255,0,255,0.06) !important;
    border: 1px solid rgba(255,0,255,0.15) !important;
    color: var(--text-secondary) !important;
    font-family: 'Share Tech Mono', monospace !important;
    font-size: 0.8rem !important;
    transition: all 0.3s ease !important;
}
.examples-row button:hover {
    background: rgba(255,0,255,0.12) !important;
    border-color: var(--neon-pink) !important;
    color: var(--neon-cyan) !important;
}

/* ═══ SECTION HEADERS ═══ */
.section-header {
    font-family: 'Orbitron', monospace;
    font-size: 0.75rem;
    color: var(--neon-pink);
    letter-spacing: 4px;
    text-transform: uppercase;
    text-shadow: 0 0 8px rgba(255,0,255,0.4);
    text-align: center;
    padding: 10px 0 5px;
    border-bottom: 1px solid rgba(255,0,255,0.1);
    margin-bottom: 10px;
}

/* ═══ FOOTER ═══ */
.cyber-footer {
    text-align: center;
    padding: 20px;
    font-family: 'Share Tech Mono', monospace;
    font-size: 0.7rem;
    color: var(--text-dim);
    border-top: 1px solid rgba(255,0,255,0.08);
    margin-top: 15px;
    letter-spacing: 2px;
}

/* ═══════════════════════════════════════ */
/*  KEYFRAME ANIMATIONS                  */
/* ═══════════════════════════════════════ */

@keyframes bgShift {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.97; }
}

@keyframes scanMove {
    0% { transform: translateY(0); }
    100% { transform: translateY(4px); }
}

@keyframes textFlicker {
    0%, 19.999%, 22%, 62.999%, 64%, 64.999%, 70%, 100% { opacity: 1; }
    20%, 21.999%, 63%, 63.999%, 65%, 69.999% { opacity: 0.88; }
}

@keyframes glitch1 {
    0% { transform: translate(0); }
    20% { transform: translate(-3px, 1px); }
    40% { transform: translate(3px, -1px); }
    60% { transform: translate(-1px, 2px); }
    80% { transform: translate(2px, -1px); }
    100% { transform: translate(0); }
}

@keyframes glitch2 {
    0% { transform: translate(0); }
    20% { transform: translate(2px, -1px); }
    40% { transform: translate(-3px, 1px); }
    60% { transform: translate(1px, -2px); }
    80% { transform: translate(-2px, 1px); }
    100% { transform: translate(0); }
}

@keyframes dotPulse {
    0%, 100% { opacity: 1; box-shadow: 0 0 6px currentColor; }
    50% { opacity: 0.5; box-shadow: 0 0 12px currentColor; }
}

@keyframes borderSpin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
}

@keyframes dataStream {
    0% { transform: translateX(-100%); }
    100% { transform: translateX(100%); }
}

@keyframes progressGlow {
    0%, 100% { box-shadow: 0 0 10px rgba(255,0,255,0.4); }
    50% { box-shadow: 0 0 25px rgba(0,255,255,0.6); }
}

@keyframes neonBreath {
    0%, 100% { text-shadow: 0 0 5px currentColor, 0 0 10px currentColor; }
    50% { text-shadow: 0 0 10px currentColor, 0 0 20px currentColor, 0 0 40px currentColor; }
}

/* ═══ RESPONSIVE ═══ */
@media (max-width: 768px) {
    .glitch-title { font-size: 1.8rem !important; letter-spacing: 3px; }
    .cyber-subtitle { font-size: 0.8rem !important; letter-spacing: 4px; }
    .status-bar { flex-direction: column; align-items: center; gap: 8px; }
    .tabs > .tab-nav > button {
        padding: 8px 12px !important;
        font-size: 0.7rem !important;
        letter-spacing: 1px !important;
    }
}

@media (max-width: 480px) {
    .glitch-title { font-size: 1.3rem !important; }
    .cyber-subtitle { font-size: 0.65rem !important; }
}
"""

# ═══════════════════════════════════════════════════════════════
#  HTML COMPONENTS
# ═══════════════════════════════════════════════════════════════

HEADER_HTML = """
<div class="cyber-header">
    <h1 class="glitch-title" data-text="CYBERPUNK DREAM GENERATOR">CYBERPUNK DREAM GENERATOR</h1>
    <p class="cyber-subtitle">⟨ NEURAL ART ⟩ × ⟨ SYNTH MUSIC ⟩ × ⟨ CODE MATRIX ⟩</p>
    <div class="status-bar">
        <span class="status-item"><span class="status-dot online"></span> SYSTEM: ONLINE</span>
        <span class="status-item"><span class="status-dot active"></span> NEURAL CORE: ACTIVE</span>
        <span class="status-item"><span class="status-dot ready"></span> RENDER ENGINE: v2.0.77</span>
        <span class="status-item"><span class="status-dot hot"></span> QUANTUM BUFFER: READY</span>
    </div>
</div>
"""

SYSTEM_CORE_HTML = """
<div class="system-panel">
    <div class="system-ascii">
 ██████╗██╗   ██╗██████╗ ███████╗██████╗     ██████╗ ██████╗ ██████╗ ███████╗
██╔════╝╚██╗ ██╔╝██╔══██╗██╔════╝██╔══██╗   ██╔════╝██╔═══██╗██╔══██╗██╔════╝
██║      ╚████╔╝ ██████╔╝█████╗  ██████╔╝   ██║     ██║   ██║██████╔╝█████╗
██║       ╚██╔╝  ██╔══██╗██╔══╝  ██╔══██╗   ██║     ██║   ██║██╔══██╗██╔══╝
╚██████╗   ██║   ██████╔╝███████╗██║  ██║   ╚██████╗╚██████╔╝██║  ██║███████╗
 ╚═════╝   ╚═╝   ╚═════╝ ╚══════╝╚═╝  ╚═╝    ╚═════╝ ╚═════╝ ╚═╝  ╚═╝╚══════╝
    </div>
    <div style="text-align:center; font-family:'Share Tech Mono',monospace; color:#8888bb; font-size:0.85rem; letter-spacing:4px; margin-bottom:20px;">
        D R E A M &nbsp; G E N E R A T O R &nbsp; v 2 . 0 . 7 7
    </div>
    <div class="system-stats">
        <div class="stat-card">
            <h4>⚡ Neural Core</h4>
            <div class="stat-line"><span>Processing Power</span><span class="stat-value">████████████░░ 92%</span></div>
            <div class="stat-line"><span>Dream Threads</span><span class="stat-value green">4,096 ACTIVE</span></div>
            <div class="stat-line"><span>Synth Modules</span><span class="stat-value green">ALL ONLINE</span></div>
            <div class="stat-line"><span>Render Pipeline</span><span class="stat-value">QUANTUM-V2</span></div>
        </div>
        <div class="stat-card">
            <h4>💾 Memory Matrix</h4>
            <div class="stat-line"><span>Quantum Buffer</span><span class="stat-value">42.7 TB</span></div>
            <div class="stat-line"><span>Dream Cache</span><span class="stat-value pink">18.3 TB</span></div>
            <div class="stat-line"><span>Code Repository</span><span class="stat-value">7.2 TB</span></div>
            <div class="stat-line"><span>Music Archive</span><span class="stat-value yellow">12.8 TB</span></div>
        </div>
        <div class="stat-card">
            <h4>🌐 Network Status</h4>
            <div class="stat-line"><span>Uplink</span><span class="stat-value green">ENCRYPTED</span></div>
            <div class="stat-line"><span>Latency</span><span class="stat-value">0.42ms</span></div>
            <div class="stat-line"><span>Bandwidth</span><span class="stat-value">∞ TB/s</span></div>
            <div class="stat-line"><span>Firewalls</span><span class="stat-value green">ICE-7 ACTIVE</span></div>
        </div>
        <div class="stat-card">
            <h4>🔮 Generation Stats</h4>
            <div class="stat-line"><span>Dreams Created</span><span class="stat-value pink">∞</span></div>
            <div class="stat-line"><span>Tracks Forged</span><span class="stat-value yellow">∞</span></div>
            <div class="stat-line"><span>Code Compiled</span><span class="stat-value green">∞</span></div>
            <div class="stat-line"><span>Uptime</span><span class="stat-value">99.97%</span></div>
        </div>
    </div>
    <div style="text-align:center; margin-top:20px;">
        <div style="font-family:'Share Tech Mono',monospace; color:#555580; font-size:0.75rem; letter-spacing:2px;">
            ◈ POWERED BY QUANTUM NEURAL NETWORKS ◈<br>
            ◈ PROCEDURAL GENERATION ENGINE ◈<br>
            ◈ NO EXTERNAL APIs — PURE MATHEMATICS ◈
        </div>
    </div>
</div>
"""

FOOTER_HTML = """
<div class="cyber-footer">
    ◈ CYBERPUNK DREAM GENERATOR v2.0.77 ◈ NEURAL ART × SYNTH MUSIC × CODE MATRIX ◈<br>
    POWERED BY PURE MATHEMATICS — NO EXTERNAL APIs REQUIRED<br>
    ⟨ DEPLOY ON HUGGING FACE SPACES ⟩
</div>
"""

# ═══════════════════════════════════════════════════════════════
#  ART GENERATION ENGINE — HELPER FUNCTIONS
# ═══════════════════════════════════════════════════════════════

def prompt_to_seed(prompt: str) -> int:
    return int(hashlib.md5(prompt.encode("utf-8")).hexdigest()[:8], 16)


def lerp_color(c1: tuple, c2: tuple, t: float) -> tuple:
    t = max(0.0, min(1.0, t))
    return tuple(int(c1[i] + (c2[i] - c1[i]) * t) for i in range(3))


def draw_gradient_rect(draw, x0, y0, x1, y1, color_top, color_bottom):
    for y in range(y0, y1):
        t = (y - y0) / max(1, (y1 - y0 - 1))
        c = lerp_color(color_top, color_bottom, t)
        draw.line([(x0, y), (x1, y)], fill=c)


def add_scanlines(img, spacing=2, alpha=25):
    overlay = Image.new("RGBA", img.size, (0, 0, 0, 0))
    draw = ImageDraw.Draw(overlay)
    for y in range(0, img.height, spacing):
        draw.line([(0, y), (img.width, y)], fill=(0, 0, 0, alpha))
    if img.mode != "RGBA":
        img = img.convert("RGBA")
    return Image.alpha_composite(img, overlay).convert("RGB")


def add_noise(img, amount=15):
    arr = np.array(img, dtype=np.int16)
    noise = np.random.randint(-amount, amount + 1, arr.shape, dtype=np.int16)
    arr = np.clip(arr + noise, 0, 255).astype(np.uint8)
    return Image.fromarray(arr)


def add_vignette(img, strength=0.5):
    w, h = img.size
    vignette = Image.new("L", (w, h), 255)
    draw = ImageDraw.Draw(vignette)
    cx, cy = w // 2, h // 2
    max_r = math.sqrt(cx * cx + cy * cy)
    for r in range(int(max_r), 0, -2):
        alpha = int(255 * (1.0 - strength * (r / max_r) ** 2))
        alpha = max(0, min(255, alpha))
        draw.ellipse((cx - r, cy - r, cx + r, cy + r), fill=alpha)
    img_rgba = img.convert("RGBA")
    black = Image.new("RGBA", (w, h), (0, 0, 0, 255))
    return Image.composite(img_rgba, black, vignette).convert("RGB")


def create_glow_layer(size, elements, blur_radius=25):
    glow = Image.new("RGB", size, (0, 0, 0))
    draw = ImageDraw.Draw(glow)
    for elem in elements:
        if elem["type"] == "circle":
            x, y, r = elem["x"], elem["y"], elem["r"]
            c = elem.get("color", (255, 0, 255))
            draw.ellipse((x - r, y - r, x + r, y + r), fill=c)
        elif elem["type"] == "line":
            draw.line(elem["coords"], fill=elem.get("color", (255, 0, 255)), width=elem.get("width", 2))
    return glow.filter(ImageFilter.GaussianBlur(radius=blur_radius))


# ═══════════════════════════════════════════════════════════════
#  ART STYLE: SYNTHWAVE SUNSET
# ═══════════════════════════════════════════════════════════════

def generate_synthwave(w, h, pal):
    img = Image.new("RGB", (w, h), (5, 0, 20))
    draw = ImageDraw.Draw(img)
    horizon = int(h * 0.48)

    # Sky gradient
    colors_sky = [
        (0.0, (5, 0, 30)),
        (0.25, (15, 0, 50)),
        (0.45, (80, 0, 100)),
        (0.65, (180, 20, 80)),
        (0.85, (255, 80, 50)),
        (1.0, (255, 180, 80)),
    ]
    for y in range(horizon):
        t = y / max(1, horizon - 1)
        for i in range(len(colors_sky) - 1):
            if colors_sky[i][0] <= t <= colors_sky[i + 1][0]:
                sub_t = (t - colors_sky[i][0]) / (colors_sky[i + 1][0] - colors_sky[i][0])
                c = lerp_color(colors_sky[i][1], colors_sky[i + 1][1], sub_t)
                draw.line([(0, y), (w, y)], fill=c)
                break

    # Stars
    for _ in range(150):
        sx = random.randint(0, w)
        sy = random.randint(0, int(horizon * 0.5))
        brightness = random.randint(120, 255)
        size = random.choice([0, 0, 0, 1])
        if size == 0:
            draw.point((sx, sy), fill=(brightness, brightness, brightness + 20))
        else:
            draw.ellipse((sx - 1, sy - 1, sx + 1, sy + 1), fill=(brightness, brightness, brightness))

    # Sun
    sun_cx, sun_cy = w // 2, int(h * 0.40)
    sun_r = int(h * 0.14)
    glow_elements = [{"type": "circle", "x": sun_cx, "y": sun_cy, "r": sun_r * 3, "color": (80, 20, 40)}]
    glow = create_glow_layer((w, h), glow_elements, blur_radius=40)
    img = ImageChops.add(img, glow)
    draw = ImageDraw.Draw(img)

    for yo in range(-sun_r, sun_r):
        x_half = int(math.sqrt(max(0, sun_r ** 2 - yo ** 2)))
        t = (yo + sun_r) / (2 * sun_r)
        r = 255
        g = int(255 - 180 * t)
        b = int(80 - 70 * t)
        row_y = sun_cy + yo
        if yo > 0 and (yo % 6 < 3):
            continue
        draw.line([(sun_cx - x_half, row_y), (sun_cx + x_half, row_y)], fill=(r, g, b))

    # Mountains
    peaks = []
    x = 0
    while x < w:
        peak_h = random.randint(int(horizon * 0.6), int(horizon * 0.85))
        peak_w = random.randint(80, 200)
        peaks.append((x, peak_h, peak_w))
        x += peak_w // 2 + random.randint(20, 60)
    for px, ph, pw in peaks:
        for offset in range(pw):
            mx = px + offset
            if 0 <= mx < w:
                t = abs(offset - pw // 2) / (pw // 2)
                my = int(ph + (horizon - ph) * t)
                for y in range(my, horizon):
                    draw.point((mx, y), fill=lerp_color((15, 5, 35), (25, 10, 50), (y - my) / max(1, horizon - my)))

    # Grid floor
    p = pal["primary"]
    s = pal["secondary"]
    for i in range(30):
        t = (i / 30) ** 1.8
        y = horizon + int(t * (h - horizon))
        alpha = max(40, 255 - int(200 * (1 - t)))
        c = lerp_color((30, 0, 40), p, 0.3 + 0.4 * t)
        draw.line([(0, y), (w, y)], fill=c, width=1)

    vp_x = w // 2
    for i in range(-15, 16):
        bx = vp_x + i * (w // 12)
        c = lerp_color((0, 30, 40), s, 0.2 + 0.3 * abs(i) / 15)
        draw.line([(vp_x, horizon), (bx, h)], fill=c, width=1)

    return img


# ═══════════════════════════════════════════════════════════════
#  ART STYLE: CYBER CITY
# ═══════════════════════════════════════════════════════════════

def generate_cyber_city(w, h, pal):
    img = Image.new("RGB", (w, h), (5, 5, 15))
    draw = ImageDraw.Draw(img)

    # Sky gradient
    draw_gradient_rect(draw, 0, 0, w, int(h * 0.55), (5, 5, 20), (15, 8, 35))

    # Moon
    mx, my, mr = int(w * 0.75), int(h * 0.12), int(h * 0.05)
    glow = create_glow_layer((w, h), [{"type": "circle", "x": mx, "y": my, "r": mr * 4, "color": (20, 20, 50)}], 30)
    img = ImageChops.add(img, glow)
    draw = ImageDraw.Draw(img)
    draw.ellipse((mx - mr, my - mr, mx + mr, my + mr), fill=(200, 200, 230))

    # Buildings
    ground = int(h * 0.85)
    buildings = []
    bx = 0
    while bx < w:
        bw = random.randint(30, 90)
        bh = random.randint(int(h * 0.15), int(h * 0.55))
        by = ground - bh
        buildings.append((bx, by, bw, bh))
        bx += bw + random.randint(-5, 8)

    for bx, by, bw, bh in sorted(buildings, key=lambda b: b[3]):
        shade = random.randint(12, 28)
        tint = random.choice([(shade, shade, shade + 8), (shade, shade + 3, shade + 5), (shade + 5, shade, shade + 3)])
        draw.rectangle([bx, by, bx + bw, ground], fill=tint)
        draw.rectangle([bx, by, bx + bw, ground], outline=(tint[0] + 10, tint[1] + 10, tint[2] + 10), width=1)

        # Windows
        win_w, win_h = 4, 5
        for wy in range(by + 6, by + bh - 5, win_h + 3):
            for wx in range(bx + 4, bx + bw - 4, win_w + 3):
                if random.random() < 0.45:
                    wc = random.choice([
                        (255, 255, 180), (200, 220, 255), (255, 200, 100),
                        (0, 255, 255), (255, 100, 200), (100, 255, 100),
                    ])
                    brightness = random.uniform(0.3, 1.0)
                    wc = tuple(int(c * brightness) for c in wc)
                    draw.rectangle([wx, wy, wx + win_w, wy + win_h], fill=wc)

    # Neon signs
    sign_colors = [pal["primary"], pal["secondary"], pal["accent"], (255, 0, 100), (0, 255, 200)]
    sign_glows = []
    for _ in range(random.randint(5, 12)):
        bx, by, bw, bh = random.choice(buildings)
        sx = bx + random.randint(3, max(4, bw - 20))
        sy = by + random.randint(5, max(6, bh // 3))
        sw = random.randint(10, min(30, bw - 6))
        sh = random.randint(4, 12)
        sc = random.choice(sign_colors)
        draw.rectangle([sx, sy, sx + sw, sy + sh], fill=sc)
        sign_glows.append({"type": "circle", "x": sx + sw // 2, "y": sy + sh // 2, "r": sw, "color": tuple(c // 4 for c in sc)})

    glow = create_glow_layer((w, h), sign_glows, 15)
    img = ImageChops.add(img, glow)
    draw = ImageDraw.Draw(img)

    # Street
    draw_gradient_rect(draw, 0, ground, w, h, (10, 8, 20), (5, 3, 12))

    # Rain
    for _ in range(random.randint(200, 500)):
        rx = random.randint(0, w)
        ry = random.randint(0, h)
        rl = random.randint(5, 20)
        ra = random.randint(30, 80)
        draw.line([(rx, ry), (rx + 1, ry + rl)], fill=(150, 180, 220, ra), width=1)

    # Fog
    fog = Image.new("RGB", (w, h), (0, 0, 0))
    fog_draw = ImageDraw.Draw(fog)
    for y in range(ground - 50, ground + 20):
        t = (y - (ground - 50)) / 70
        alpha = int(30 * t)
        fog_draw.line([(0, y), (w, y)], fill=(alpha, alpha, alpha + 5))
    fog = fog.filter(ImageFilter.GaussianBlur(radius=10))
    img = ImageChops.add(img, fog)

    return img


# ═══════════════════════════════════════════════════════════════
#  ART STYLE: NEURAL NETWORK
# ═══════════════════════════════════════════════════════════════

def generate_neural_net(w, h, pal):
    img = Image.new("RGB", (w, h), (5, 5, 12))
    draw = ImageDraw.Draw(img)

    # Subtle grid
    for x in range(0, w, 30):
        draw.line([(x, 0), (x, h)], fill=(10, 10, 20), width=1)
    for y in range(0, h, 30):
        draw.line([(0, y), (w, y)], fill=(10, 10, 20), width=1)

    # Network layers
    n_layers = random.randint(4, 6)
    nodes_per = [random.randint(3, 7) for _ in range(n_layers)]
    nodes_per[0] = random.randint(3, 5)
    nodes_per[-1] = random.randint(2, 4)
    margin_x = int(w * 0.1)
    margin_y = int(h * 0.12)
    layer_spacing = (w - 2 * margin_x) / max(1, n_layers - 1)
    node_positions = []

    for li in range(n_layers):
        lx = margin_x + int(li * layer_spacing)
        nn = nodes_per[li]
        node_spacing = (h - 2 * margin_y) / max(1, nn - 1) if nn > 1 else 0
        layer = []
        for ni in range(nn):
            ny = margin_y + int(ni * node_spacing) if nn > 1 else h // 2
            layer.append((lx, ny))
        node_positions.append(layer)

    # Draw connections
    glow_elems = []
    for li in range(len(node_positions) - 1):
        for n1 in node_positions[li]:
            for n2 in node_positions[li + 1]:
                weight = random.random()
                if weight > 0.6:
                    c = lerp_color((0, 40, 60), pal["primary"], weight)
                    draw.line([n1, n2], fill=c, width=1)
                    if weight > 0.85:
                        glow_elems.append({"type": "line", "coords": [n1, n2], "color": tuple(v // 5 for v in pal["primary"]), "width": 3})
                else:
                    c = lerp_color((15, 10, 25), (40, 20, 60), weight)
                    draw.line([n1, n2], fill=c, width=1)

    # Glow
    if glow_elems:
        glow = create_glow_layer((w, h), glow_elems, 8)
        img = ImageChops.add(img, glow)
        draw = ImageDraw.Draw(img)

    # Draw nodes
    layer_colors = [pal["accent"], pal["secondary"], pal["primary"], pal["secondary"], pal["accent"], pal["primary"]]
    node_glows = []
    for li, layer in enumerate(node_positions):
        nc = layer_colors[li % len(layer_colors)]
        for nx, ny in layer:
            r = random.randint(6, 10)
            draw.ellipse((nx - r, ny - r, nx + r, ny + r), fill=nc, outline=(255, 255, 255))
            node_glows.append({"type": "circle", "x": nx, "y": ny, "r": r * 3, "color": tuple(v // 6 for v in nc)})

    glow = create_glow_layer((w, h), node_glows, 12)
    img = ImageChops.add(img, glow)
    draw = ImageDraw.Draw(img)

    # Data flow particles
    for li in range(len(node_positions) - 1):
        for _ in range(random.randint(2, 5)):
            n1 = random.choice(node_positions[li])
            n2 = random.choice(node_positions[li + 1])
            t = random.random()
            px = int(n1[0] + (n2[0] - n1[0]) * t)
            py = int(n1[1] + (n2[1] - n1[1]) * t)
            pc = pal["glow"]
            draw.ellipse((px - 3, py - 3, px + 3, py + 3), fill=pc)

    # Labels
    labels = ["INPUT", "HIDDEN", "HIDDEN", "HIDDEN", "PROCESS", "OUTPUT"]
    for li, layer in enumerate(node_positions):
        lx = layer[0][0]
        label = labels[li] if li < len(labels) else "LAYER"
        draw.text((lx - 20, h - margin_y + 15), label, fill=pal["primary"])

    return img


# ═══════════════════════════════════════════════════════════════
#  ART STYLE: DIGITAL DREAMS
# ═══════════════════════════════════════════════════════════════

def generate_digital_dreams(w, h, pal):
    img = Image.new("RGB", (w, h), (5, 0, 15))
    draw = ImageDraw.Draw(img)
    draw_gradient_rect(draw, 0, 0, w, h, pal["bg_top"], pal["bg_bottom"])

    # Flowing bezier curves
    for _ in range(random.randint(12, 25)):
        pts = [(random.randint(0, w), random.randint(0, h)) for _ in range(4)]
        color = random.choice([pal["primary"], pal["secondary"], pal["accent"], pal["glow"]])
        prev = pts[0]
        for ti in range(1, 60):
            t = ti / 60
            u = 1 - t
            x = int(u ** 3 * pts[0][0] + 3 * u ** 2 * t * pts[1][0] + 3 * u * t ** 2 * pts[2][0] + t ** 3 * pts[3][0])
            y = int(u ** 3 * pts[0][1] + 3 * u ** 2 * t * pts[1][1] + 3 * u * t ** 2 * pts[2][1] + t ** 3 * pts[3][1])
            alpha_fac = math.sin(t * math.pi)
            c = tuple(max(0, min(255, int(v * (0.3 + 0.7 * alpha_fac)))) for v in color)
            draw.line([prev, (x, y)], fill=c, width=random.choice([1, 1, 2]))
            prev = (x, y)

    # Orbital rings
    for _ in range(random.randint(3, 7)):
        cx = random.randint(w // 4, 3 * w // 4)
        cy = random.randint(h // 4, 3 * h // 4)
        r = random.randint(30, 120)
        color = random.choice([pal["primary"], pal["secondary"]])
        segments = 80
        for si in range(segments):
            a1 = 2 * math.pi * si / segments
            a2 = 2 * math.pi * (si + 1) / segments
            x1, y1 = cx + int(r * math.cos(a1)), cy + int(r * math.sin(a1))
            x2, y2 = cx + int(r * math.cos(a2)), cy + int(r * math.sin(a2))
            fade = abs(math.sin(a1 * 2))
            c = tuple(max(0, min(255, int(v * (0.15 + 0.5 * fade)))) for v in color)
            draw.line([(x1, y1), (x2, y2)], fill=c, width=1)

    # Particles
    particle_glows = []
    for _ in range(random.randint(150, 350)):
        px = random.randint(0, w)
        py = random.randint(0, h)
        pr = random.randint(1, 3)
        pc = random.choice([pal["primary"], pal["secondary"], pal["accent"], pal["glow"]])
        brightness = random.uniform(0.3, 1.0)
        c = tuple(int(v * brightness) for v in pc)
        draw.ellipse((px - pr, py - pr, px + pr, py + pr), fill=c)
        if pr >= 2:
            particle_glows.append({"type": "circle", "x": px, "y": py, "r": pr * 5, "color": tuple(v // 8 for v in pc)})

    # Glowing orbs
    for _ in range(random.randint(3, 6)):
        ox = random.randint(50, w - 50)
        oy = random.randint(50, h - 50)
        orr = random.randint(15, 35)
        oc = random.choice([pal["primary"], pal["secondary"], pal["glow"]])
        particle_glows.append({"type": "circle", "x": ox, "y": oy, "r": orr * 4, "color": tuple(v // 4 for v in oc)})
        draw.ellipse((ox - orr, oy - orr, ox + orr, oy + orr), fill=oc)

    glow = create_glow_layer((w, h), particle_glows, 15)
    img = ImageChops.add(img, glow)

    return img


# ═══════════════════════════════════════════════════════════════
#  ART STYLE: GLITCH ART
# ═══════════════════════════════════════════════════════════════

def generate_glitch_art(w, h, pal):
    img = Image.new("RGB", (w, h), (5, 5, 10))
    draw = ImageDraw.Draw(img)

    # Colorful geometric base
    for _ in range(random.randint(15, 30)):
        shape = random.choice(["rect", "line", "triangle"])
        c = random.choice([pal["primary"], pal["secondary"], pal["accent"], pal["glow"],
                           (255, 0, 100), (100, 0, 255), (0, 200, 200)])
        brightness = random.uniform(0.4, 1.0)
        c = tuple(int(v * brightness) for v in c)
        if shape == "rect":
            x1, y1 = random.randint(0, w), random.randint(0, h)
            x2, y2 = x1 + random.randint(20, 200), y1 + random.randint(20, 150)
            draw.rectangle([x1, y1, x2, y2], fill=c if random.random() > 0.4 else None, outline=c, width=2)
        elif shape == "line":
            coords = [(random.randint(0, w), random.randint(0, h)) for _ in range(2)]
            draw.line(coords, fill=c, width=random.randint(1, 5))
        elif shape == "triangle":
            pts = [(random.randint(0, w), random.randint(0, h)) for _ in range(3)]
            draw.polygon(pts, fill=c if random.random() > 0.5 else None, outline=c)

    # Gradient bars
    for _ in range(random.randint(3, 8)):
        y = random.randint(0, h)
        bh = random.randint(10, 60)
        c1 = random.choice([pal["primary"], pal["secondary"], pal["accent"]])
        c2 = random.choice([pal["primary"], pal["secondary"], pal["accent"]])
        draw_gradient_rect(draw, 0, y, w, min(y + bh, h), c1, c2)

    # RGB channel split
    r_ch, g_ch, b_ch = img.split()
    offset_r = random.randint(5, 15)
    offset_b = random.randint(5, 15)
    r_shifted = Image.new("L", (w, h), 0)
    b_shifted = Image.new("L", (w, h), 0)
    r_shifted.paste(r_ch, (-offset_r, random.randint(-2, 2)))
    b_shifted.paste(b_ch, (offset_b, random.randint(-2, 2)))
    img = Image.merge("RGB", (r_shifted, g_ch, b_shifted))
    draw = ImageDraw.Draw(img)

    # Horizontal glitch strips
    for _ in range(random.randint(10, 30)):
        sy = random.randint(0, h)
        sh = random.randint(2, 25)
        strip = img.crop((0, sy, w, min(sy + sh, h)))
        offset = random.randint(-50, 50)
        img.paste(strip, (offset, sy))
    draw = ImageDraw.Draw(img)

    # Block corruption
    for _ in range(random.randint(5, 15)):
        bx = random.randint(0, w - 20)
        by = random.randint(0, h - 10)
        bw = random.randint(10, 80)
        bh = random.randint(3, 20)
        c = random.choice([pal["primary"], pal["secondary"], pal["accent"], (0, 0, 0), (255, 255, 255)])
        draw.rectangle([bx, by, bx + bw, by + bh], fill=c)

    # Strong scan lines
    for y in range(0, h, 3):
        draw.line([(0, y), (w, y)], fill=(0, 0, 0), width=1)

    return img


# ═══════════════════════════════════════════════════════════════
#  ART STYLE: QUANTUM REALM
# ═══════════════════════════════════════════════════════════════

def generate_quantum_realm(w, h, pal):
    img = Image.new("RGB", (w, h), (3, 0, 8))
    draw = ImageDraw.Draw(img)
    cx, cy = w // 2, h // 2

    # Background radial gradient
    max_r = int(math.sqrt(cx ** 2 + cy ** 2))
    for r in range(max_r, 0, -3):
        t = r / max_r
        c = lerp_color((3, 0, 8), pal["bg_bottom"], 1 - t)
        draw.ellipse((cx - r, cy - r, cx + r, cy + r), fill=c)

    # Concentric rings
    ring_glows = []
    for i in range(random.randint(8, 18)):
        r = int(30 + i * (min(w, h) * 0.4) / 18)
        color = random.choice([pal["primary"], pal["secondary"], pal["accent"]])
        alpha = random.uniform(0.2, 0.7)
        c = tuple(int(v * alpha) for v in color)
        segments = 120
        for si in range(segments):
            a1 = 2 * math.pi * si / segments
            a2 = 2 * math.pi * (si + 1) / segments
            x1, y1 = cx + int(r * math.cos(a1)), cy + int(r * math.sin(a1))
            x2, y2 = cx + int(r * math.cos(a2)), cy + int(r * math.sin(a2))
            draw.line([(x1, y1), (x2, y2)], fill=c, width=1)

    # Spiral arms
    for arm in range(random.randint(2, 4)):
        offset = arm * (2 * math.pi / random.randint(2, 4))
        color = random.choice([pal["primary"], pal["secondary"], pal["glow"]])
        for ti in range(300):
            t = ti / 300
            angle = offset + t * 6 * math.pi
            r = t * min(w, h) * 0.45
            x = int(cx + r * math.cos(angle))
            y = int(cy + r * math.sin(angle))
            size = max(1, int(3 * (1 - t)))
            alpha = 1 - t * 0.7
            c = tuple(int(v * alpha) for v in color)
            draw.ellipse((x - size, y - size, x + size, y + size), fill=c)

    # Energy orbs
    orb_glows = []
    for _ in range(random.randint(3, 7)):
        angle = random.uniform(0, 2 * math.pi)
        dist = random.uniform(0.1, 0.4) * min(w, h)
        ox = int(cx + dist * math.cos(angle))
        oy = int(cy + dist * math.sin(angle))
        orr = random.randint(5, 15)
        oc = random.choice([pal["primary"], pal["secondary"], pal["glow"]])
        draw.ellipse((ox - orr, oy - orr, ox + orr, oy + orr), fill=oc)
        orb_glows.append({"type": "circle", "x": ox, "y": oy, "r": orr * 5, "color": tuple(v // 5 for v in oc)})

    # Central orb
    cr = int(min(w, h) * 0.04)
    draw.ellipse((cx - cr, cy - cr, cx + cr, cy + cr), fill=pal["glow"])
    orb_glows.append({"type": "circle", "x": cx, "y": cy, "r": cr * 8, "color": tuple(v // 3 for v in pal["glow"])})

    glow = create_glow_layer((w, h), orb_glows, 20)
    img = ImageChops.add(img, glow)

    # Radial beams
    draw = ImageDraw.Draw(img)
    for _ in range(random.randint(6, 12)):
        angle = random.uniform(0, 2 * math.pi)
        length = random.uniform(0.2, 0.5) * min(w, h)
        x2 = int(cx + length * math.cos(angle))
        y2 = int(cy + length * math.sin(angle))
        c = tuple(int(v * 0.15) for v in pal["glow"])
        draw.line([(cx, cy), (x2, y2)], fill=c, width=1)

    return img


# ═══════════════════════════════════════════════════════════════
#  ART GENERATION — MAIN DISPATCHER
# ═══════════════════════════════════════════════════════════════

ART_STYLES = [
    "🌅 Synthwave Sunset",
    "🏙️ Cyber City",
    "🧠 Neural Network",
    "💫 Digital Dreams",
    "📺 Glitch Art",
    "🔮 Quantum Realm",
]

def generate_dream_art(prompt, style, width, height, palette_name, intensity, progress=gr.Progress()):
    if not prompt.strip():
        prompt = "cyberpunk dream neon city"

    seed = prompt_to_seed(prompt)
    random.seed(seed)
    np.random.seed(seed % (2 ** 32))

    pal = PALETTES.get(palette_name, PALETTES["🔮 Neon Noir"])
    w, h = int(width), int(height)

    progress(0.1, desc="⚡ INITIALIZING NEURAL DREAM MATRIX...")
    time.sleep(0.1)

    progress(0.2, desc="🔮 LOADING QUANTUM RENDER CORE...")

    style_map = {
        "🌅 Synthwave Sunset": generate_synthwave,
        "🏙️ Cyber City": generate_cyber_city,
        "🧠 Neural Network": generate_neural_net,
        "💫 Digital Dreams": generate_digital_dreams,
        "📺 Glitch Art": generate_glitch_art,
        "🔮 Quantum Realm": generate_quantum_realm,
    }

    gen_fn = style_map.get(style, generate_synthwave)
    progress(0.4, desc="🎨 RENDERING QUANTUM PIXELS...")
    img = gen_fn(w, h, pal)

    progress(0.7, desc="✨ APPLYING NEON GLOW EFFECTS...")

    # Post-processing based on intensity
    if intensity > 1.0:
        enhancer = ImageEnhance.Color(img)
        img = enhancer.enhance(intensity)

    # Apply neon boost
    if intensity > 0.5:
        glow_layer = img.filter(ImageFilter.GaussianBlur(radius=int(5 * intensity)))
        enhancer = ImageEnhance.Brightness(glow_layer)
        glow_layer = enhancer.enhance(0.3 * intensity)
        img = ImageChops.add(img, glow_layer)

    progress(0.85, desc="📡 ADDING ATMOSPHERIC EFFECTS...")
    img = add_scanlines(img, spacing=3, alpha=int(15 * intensity))
    img = add_noise(img, amount=int(8 * intensity))
    img = add_vignette(img, strength=0.4)

    progress(0.95, desc="💾 ENCODING DREAM OUTPUT...")
    time.sleep(0.05)
    progress(1.0, desc="🔥 DREAM MATERIALIZED!")

    log = (
        f"╔══ NEURAL DREAM LOG ══════════════════════════╗\n"
        f"║ PROMPT: {prompt[:42]}\n"
        f"║ STYLE: {style}\n"
        f"║ RESOLUTION: {w}×{h}\n"
        f"║ PALETTE: {palette_name}\n"
        f"║ INTENSITY: {intensity:.1f}\n"
        f"║ SEED: {seed}\n"
        f"║ STATUS: ███████████████ COMPLETE\n"
        f"╚═════════════════════════════════════════════╝"
    )
    return img, log


# ═══════════════════════════════════════════════════════════════
#  MUSIC GENERATION ENGINE — OSCILLATORS & UTILITIES
# ═══════════════════════════════════════════════════════════════

SR = 44100


def sine_wave(freq, duration, sr=SR):
    t = np.linspace(0, duration, int(sr * duration), False)
    return np.sin(2 * np.pi * freq * t)


def saw_wave(freq, duration, sr=SR):
    t = np.linspace(0, duration, int(sr * duration), False)
    return 2.0 * (t * freq - np.floor(0.5 + t * freq))


def square_wave(freq, duration, sr=SR):
    return np.sign(sine_wave(freq, duration, sr))


def triangle_wave(freq, duration, sr=SR):
    t = np.linspace(0, duration, int(sr * duration), False)
    return 2.0 * np.abs(2.0 * (t * freq - np.floor(t * freq + 0.5))) - 1.0


def white_noise(duration, sr=SR):
    return np.random.uniform(-1, 1, int(sr * duration))


def adsr_envelope(duration, sr=SR, a=0.01, d=0.05, s_level=0.7, r=0.1):
    total = int(sr * duration)
    env = np.zeros(total)
    a_samp = min(int(sr * a), total)
    d_samp = min(int(sr * d), total - a_samp)
    r_samp = min(int(sr * r), total)
    s_samp = max(0, total - a_samp - d_samp - r_samp)

    idx = 0
    if a_samp > 0:
        env[idx:idx + a_samp] = np.linspace(0, 1, a_samp)
        idx += a_samp
    if d_samp > 0:
        env[idx:idx + d_samp] = np.linspace(1, s_level, d_samp)
        idx += d_samp
    if s_samp > 0:
        env[idx:idx + s_samp] = s_level
        idx += s_samp
    if r_samp > 0:
        remaining = total - idx
        if remaining > 0:
            env[idx:idx + remaining] = np.linspace(s_level, 0, remaining)

    return env


def simple_lowpass(signal, alpha=0.05):
    filtered = np.zeros_like(signal)
    filtered[0] = signal[0]
    for i in range(1, len(signal)):
        filtered[i] = alpha * signal[i] + (1 - alpha) * filtered[i - 1]
    return filtered


def simple_reverb(signal, sr=SR, decay=0.3, delay_ms=80):
    delay_samp = int(sr * delay_ms / 1000)
    result = signal.copy()
    if delay_samp < len(signal):
        result[delay_samp:] += decay * signal[:-delay_samp]
    if 2 * delay_samp < len(signal):
        result[2 * delay_samp:] += decay * 0.5 * signal[:-2 * delay_samp]
    return result


def get_chord_freqs(root_name, chord_type="minor", octave=3):
    root_idx = SCALE_NOTES.index(root_name)
    intervals = CHORD_PATTERNS.get(chord_type, [0, 3, 7])
    freqs = []
    for interval in intervals:
        note_idx = (root_idx + interval) % 12
        note_name = SCALE_NOTES[note_idx]
        oct = octave + (root_idx + interval) // 12
        oct = min(oct, 5)
        freqs.append(NOTE_FREQS[note_name][oct])
    return freqs


def make_kick(duration=0.3, sr=SR):
    t = np.linspace(0, duration, int(sr * duration), False)
    freq_sweep = 150 * np.exp(-t * 20) + 40
    phase = np.cumsum(2 * np.pi * freq_sweep / sr)
    kick = np.sin(phase) * np.exp(-t * 8)
    return kick * 0.8


def make_snare(duration=0.2, sr=SR):
    t = np.linspace(0, duration, int(sr * duration), False)
    noise = white_noise(duration, sr) * np.exp(-t * 15)
    tone = np.sin(2 * np.pi * 200 * t) * np.exp(-t * 20)
    return (noise * 0.5 + tone * 0.3) * 0.7


def make_hihat(duration=0.08, sr=SR):
    t = np.linspace(0, duration, int(sr * duration), False)
    noise = white_noise(duration, sr) * np.exp(-t * 40)
    return noise * 0.3


# ═══════════════════════════════════════════════════════════════
#  MUSIC GENRE: SYNTHWAVE
# ═══════════════════════════════════════════════════════════════

def generate_synthwave_music(bpm, duration, key, sr=SR):
    beat_dur = 60.0 / bpm
    total_samples = int(sr * duration)
    output = np.zeros(total_samples)

    prog_roots = [key, SCALE_NOTES[(SCALE_NOTES.index(key) + 5) % 12],
                  SCALE_NOTES[(SCALE_NOTES.index(key) + 7) % 12],
                  SCALE_NOTES[(SCALE_NOTES.index(key) + 3) % 12]]

    bar_dur = beat_dur * 4
    bar_samples = int(sr * bar_dur)

    # Pad (detuned saws)
    pad = np.zeros(total_samples)
    for bar_i in range(int(duration / bar_dur) + 1):
        root = prog_roots[bar_i % len(prog_roots)]
        chord_freqs = get_chord_freqs(root, "minor7", octave=3)
        start = bar_i * bar_samples
        seg_dur = min(bar_dur, duration - bar_i * bar_dur)
        if seg_dur <= 0:
            break
        seg_samples = int(sr * seg_dur)
        for freq in chord_freqs:
            s1 = saw_wave(freq, seg_dur, sr)
            s2 = saw_wave(freq * 1.005, seg_dur, sr)
            chord_tone = (s1 + s2) * 0.1
            chord_tone *= adsr_envelope(seg_dur, sr, a=0.2, d=0.1, s_level=0.6, r=0.3)
            end = min(start + seg_samples, total_samples)
            pad[start:end] += chord_tone[:end - start]
    pad = simple_lowpass(pad, alpha=0.03)

    # Bass
    bass = np.zeros(total_samples)
    for bar_i in range(int(duration / bar_dur) + 1):
        root = prog_roots[bar_i % len(prog_roots)]
        freq = NOTE_FREQS[root][1]
        start = bar_i * bar_samples
        seg_dur = min(bar_dur, duration - bar_i * bar_dur)
        if seg_dur <= 0:
            break
        seg_samples = int(sr * seg_dur)
        b = saw_wave(freq, seg_dur, sr) * 0.25
        b *= adsr_envelope(seg_dur, sr, a=0.01, d=0.1, s_level=0.7, r=0.15)
        end = min(start + seg_samples, total_samples)
        bass[start:end] += b[:end - start]
    bass = simple_lowpass(bass, alpha=0.08)

    # Arpeggio
    arp = np.zeros(total_samples)
    arp_note_dur = beat_dur / 2
    note_i = 0
    for bar_i in range(int(duration / bar_dur) + 1):
        root = prog_roots[bar_i % len(prog_roots)]
        chord_freqs = get_chord_freqs(root, "minor", octave=4)
        for beat in range(8):
            idx = note_i % len(chord_freqs)
            freq = chord_freqs[idx]
            start = int((bar_i * bar_dur + beat * arp_note_dur) * sr)
            if start >= total_samples:
                break
            seg_dur = min(arp_note_dur * 0.9, (total_samples - start) / sr)
            if seg_dur <= 0:
                break
            seg_samples = int(sr * seg_dur)
            tone = square_wave(freq, seg_dur, sr) * 0.12
            tone *= adsr_envelope(seg_dur, sr, a=0.005, d=0.05, s_level=0.5, r=0.1)
            end = min(start + seg_samples, total_samples)
            arp[start:end] += tone[:end - start]
            note_i += 1

    # Drums
    drums = np.zeros(total_samples)
    beat_samples = int(sr * beat_dur)
    for beat_i in range(int(duration / beat_dur)):
        start = beat_i * beat_samples
        # Kick on 1 and 3
        if beat_i % 4 in [0, 2]:
            kick = make_kick(0.3, sr)
            end = min(start + len(kick), total_samples)
            drums[start:end] += kick[:end - start]
        # Snare on 2 and 4
        if beat_i % 4 in [1, 3]:
            snare = make_snare(0.2, sr)
            end = min(start + len(snare), total_samples)
            drums[start:end] += snare[:end - start]
        # Hi-hat on every beat
        hh = make_hihat(0.08, sr)
        end = min(start + len(hh), total_samples)
        drums[start:end] += hh[:end - start]
        # Off-beat hi-hat
        off_start = start + beat_samples // 2
        if off_start < total_samples:
            end = min(off_start + len(hh), total_samples)
            drums[off_start:end] += hh[:end - off_start] * 0.5

    output = pad + bass + arp + drums
    output = simple_reverb(output, sr, decay=0.25, delay_ms=100)
    output = np.clip(output, -1.0, 1.0)
    output *= 0.85
    return output


# ═══════════════════════════════════════════════════════════════
#  MUSIC GENRE: DARK AMBIENT
# ═══════════════════════════════════════════════════════════════

def generate_dark_ambient_music(bpm, duration, key, sr=SR):
    total_samples = int(sr * duration)
    output = np.zeros(total_samples)

    # Drone (low sine + harmonics)
    base_freq = NOTE_FREQS[key][1]
    drone = sine_wave(base_freq, duration, sr) * 0.2
    drone += sine_wave(base_freq * 2, duration, sr) * 0.08
    drone += sine_wave(base_freq * 3, duration, sr) * 0.03
    # Slow amplitude modulation
    t = np.linspace(0, duration, total_samples, False)
    mod = 0.6 + 0.4 * np.sin(2 * np.pi * 0.1 * t)
    drone *= mod

    # Pad (filtered noise evolving slowly)
    pad = white_noise(duration, sr) * 0.05
    pad = simple_lowpass(pad, alpha=0.005)
    pad_env = 0.5 + 0.5 * np.sin(2 * np.pi * 0.05 * t)
    pad *= pad_env

    # Texture tones
    texture = np.zeros(total_samples)
    for _ in range(random.randint(5, 12)):
        tone_start = random.uniform(0, duration * 0.7)
        tone_dur = random.uniform(1.0, 3.0)
        freq = base_freq * random.choice([1, 1.5, 2, 2.5, 3, 4, 5])
        start_samp = int(tone_start * sr)
        samps = min(int(tone_dur * sr), total_samples - start_samp)
        if samps <= 0:
            continue
        tone = sine_wave(freq, tone_dur, sr)[:samps] * 0.04
        env = adsr_envelope(tone_dur, sr, a=0.5, d=0.2, s_level=0.3, r=0.8)[:samps]
        tone *= env
        texture[start_samp:start_samp + samps] += tone

    output = drone + pad + texture
    output = simple_reverb(output, sr, decay=0.4, delay_ms=150)
    output = simple_reverb(output, sr, decay=0.2, delay_ms=300)
    output = np.clip(output, -1.0, 1.0)
    output *= 0.85
    return output


# ═══════════════════════════════════════════════════════════════
#  MUSIC GENRE: CYBER BASS
# ═══════════════════════════════════════════════════════════════

def generate_cyber_bass_music(bpm, duration, key, sr=SR):
    beat_dur = 60.0 / bpm
    total_samples = int(sr * duration)
    output = np.zeros(total_samples)

    base_freq = NOTE_FREQS[key][1]

    # Sub bass (sine + saw)
    bass = np.zeros(total_samples)
    beat_samples = int(sr * beat_dur)
    patterns = [[1, 0, 0, 1, 0, 0, 1, 0], [1, 0, 1, 0, 0, 1, 0, 0]]
    eighth_dur = beat_dur / 2

    for beat_i in range(int(duration / eighth_dur)):
        pattern = patterns[(beat_i // 16) % len(patterns)]
        if pattern[beat_i % 8] == 0:
            continue
        start = int(beat_i * eighth_dur * sr)
        seg_dur = min(eighth_dur * 0.8, (total_samples - start) / sr)
        if seg_dur <= 0:
            break
        seg_samp = int(sr * seg_dur)
        freq = base_freq * random.choice([1, 1, 1, 0.75, 1.5])
        b = sine_wave(freq, seg_dur, sr) * 0.3 + saw_wave(freq, seg_dur, sr) * 0.15
        b *= adsr_envelope(seg_dur, sr, a=0.005, d=0.05, s_level=0.8, r=0.1)
        end = min(start + seg_samp, total_samples)
        bass[start:end] += b[:end - start]
    bass = simple_lowpass(bass, alpha=0.1)

    # Heavy drums
    drums = np.zeros(total_samples)
    for beat_i in range(int(duration / beat_dur)):
        start = int(beat_i * beat_dur * sr)
        # Kick on every beat
        kick = make_kick(0.25, sr) * 1.3
        end = min(start + len(kick), total_samples)
        drums[start:end] += kick[:end - start]
        # Snare on 2 and 4
        if beat_i % 4 in [1, 3]:
            snare = make_snare(0.15, sr) * 1.2
            end = min(start + len(snare), total_samples)
            drums[start:end] += snare[:end - start]
        # Fast hi-hats
        for sub in range(4):
            hh_start = start + int(sub * beat_dur / 4 * sr)
            if hh_start < total_samples:
                hh = make_hihat(0.05, sr) * (0.4 if sub % 2 == 0 else 0.25)
                end = min(hh_start + len(hh), total_samples)
                drums[hh_start:end] += hh[:end - hh_start]

    # Wobble synth
    wobble = np.zeros(total_samples)
    t = np.linspace(0, duration, total_samples, False)
    wobble_raw = saw_wave(base_freq * 2, duration, sr)
    lfo_rate = bpm / 60 / 2
    lfo = 0.5 + 0.5 * np.sin(2 * np.pi * lfo_rate * t)
    # Simple filter emulation via amplitude modulation
    wobble = wobble_raw * lfo * 0.12
    wobble *= adsr_envelope(duration, sr, a=0.01, d=0.1, s_level=0.8, r=0.2)

    output = bass + drums + wobble
    output = simple_reverb(output, sr, decay=0.15, delay_ms=60)
    output = np.clip(output, -1.0, 1.0)
    output *= 0.85
    return output


# ═══════════════════════════════════════════════════════════════
#  MUSIC GENRE: NEON POP
# ═══════════════════════════════════════════════════════════════

def generate_neon_pop_music(bpm, duration, key, sr=SR):
    beat_dur = 60.0 / bpm
    total_samples = int(sr * duration)

    root_idx = SCALE_NOTES.index(key)
    prog = [0, 5, 7, 3]
    prog_roots = [SCALE_NOTES[(root_idx + iv) % 12] for iv in prog]
    bar_dur = beat_dur * 4
    bar_samples = int(sr * bar_dur)

    # Bright chord pad (square waves)
    pad = np.zeros(total_samples)
    for bar_i in range(int(duration / bar_dur) + 1):
        root = prog_roots[bar_i % len(prog_roots)]
        freqs = get_chord_freqs(root, "major", octave=4)
        start = bar_i * bar_samples
        seg_dur = min(bar_dur, duration - bar_i * bar_dur)
        if seg_dur <= 0:
            break
        seg_samp = int(sr * seg_dur)
        for freq in freqs:
            tone = square_wave(freq, seg_dur, sr) * 0.07 + triangle_wave(freq, seg_dur, sr) * 0.05
            tone *= adsr_envelope(seg_dur, sr, a=0.05, d=0.1, s_level=0.6, r=0.2)
            end = min(start + seg_samp, total_samples)
            pad[start:end] += tone[:end - start]

    # Melody
    melody = np.zeros(total_samples)
    scale = [0, 2, 4, 5, 7, 9, 11]
    note_dur = beat_dur
    for ni in range(int(duration / note_dur)):
        start = int(ni * note_dur * sr)
        seg_dur = min(note_dur * 0.8, (total_samples - start) / sr)
        if seg_dur <= 0:
            break
        degree = scale[ni % len(scale)]
        note_name = SCALE_NOTES[(root_idx + degree) % 12]
        freq = NOTE_FREQS[note_name][4]
        tone = triangle_wave(freq, seg_dur, sr) * 0.15
        tone *= adsr_envelope(seg_dur, sr, a=0.01, d=0.05, s_level=0.6, r=0.15)
        end = min(start + int(sr * seg_dur), total_samples)
        melody[start:end] += tone[:end - start]

    # Bass
    bass = np.zeros(total_samples)
    for bar_i in range(int(duration / bar_dur) + 1):
        root = prog_roots[bar_i % len(prog_roots)]
        freq = NOTE_FREQS[root][2]
        start = bar_i * bar_samples
        seg_dur = min(bar_dur, duration - bar_i * bar_dur)
        if seg_dur <= 0:
            break
        b = triangle_wave(freq, seg_dur, sr) * 0.2
        b *= adsr_envelope(seg_dur, sr, a=0.01, d=0.08, s_level=0.7, r=0.1)
        end = min(start + int(sr * seg_dur), total_samples)
        bass[start:end] += b[:end - start]

    # Drums (four-on-the-floor)
    drums = np.zeros(total_samples)
    for beat_i in range(int(duration / beat_dur)):
        start = int(beat_i * beat_dur * sr)
        kick = make_kick(0.2, sr) * 0.8
        end = min(start + len(kick), total_samples)
        drums[start:end] += kick[:end - start]
        if beat_i % 2 == 1:
            snare = make_snare(0.15, sr) * 0.6
            end = min(start + len(snare), total_samples)
            drums[start:end] += snare[:end - start]
        hh = make_hihat(0.06, sr) * 0.35
        end = min(start + len(hh), total_samples)
        drums[start:end] += hh[:end - start]

    output = pad + melody + bass + drums
    output = simple_reverb(output, sr, decay=0.2, delay_ms=80)
    output = np.clip(output, -1.0, 1.0)
    output *= 0.85
    return output


# ═══════════════════════════════════════════════════════════════
#  MUSIC GENRE: GLITCH HOP
# ═══════════════════════════════════════════════════════════════

def generate_glitch_hop_music(bpm, duration, key, sr=SR):
    beat_dur = 60.0 / bpm
    total_samples = int(sr * duration)

    base_freq = NOTE_FREQS[key][2]

    # Glitchy bass pattern
    bass = np.zeros(total_samples)
    sixteenth = beat_dur / 4
    for si in range(int(duration / sixteenth)):
        if random.random() < 0.55:
            continue
        start = int(si * sixteenth * sr)
        seg_dur = min(sixteenth * random.uniform(0.3, 0.9), (total_samples - start) / sr)
        if seg_dur <= 0:
            break
        freq = base_freq * random.choice([1, 1, 0.5, 2, 1.5, 0.75])
        waveform = random.choice([saw_wave, square_wave, triangle_wave])
        b = waveform(freq, seg_dur, sr) * 0.2
        b *= adsr_envelope(seg_dur, sr, a=0.002, d=0.02, s_level=0.7, r=0.05)
        end = min(start + int(sr * seg_dur), total_samples)
        bass[start:end] += b[:end - start]

    # Stuttered drums
    drums = np.zeros(total_samples)
    for beat_i in range(int(duration / beat_dur)):
        start = int(beat_i * beat_dur * sr)
        # Kick with stutters
        kick = make_kick(0.2, sr)
        end = min(start + len(kick), total_samples)
        drums[start:end] += kick[:end - start]
        # Random snare positions
        for sub in range(4):
            if random.random() < 0.35:
                ss = start + int(sub * beat_dur / 4 * sr)
                if ss < total_samples:
                    snare = make_snare(0.1, sr) * random.uniform(0.4, 1.0)
                    end = min(ss + len(snare), total_samples)
                    drums[ss:end] += snare[:end - ss]
        # Glitchy hi-hats
        for sub in range(8):
            if random.random() < 0.6:
                hs = start + int(sub * beat_dur / 8 * sr)
                if hs < total_samples:
                    hh = make_hihat(random.uniform(0.03, 0.1), sr) * random.uniform(0.2, 0.5)
                    end = min(hs + len(hh), total_samples)
                    drums[hs:end] += hh[:end - hs]

    # Texture
    texture = np.zeros(total_samples)
    for _ in range(random.randint(8, 20)):
        ts = int(random.uniform(0, duration) * sr)
        td = random.uniform(0.05, 0.3)
        td_samp = min(int(td * sr), total_samples - ts)
        if td_samp <= 0:
            continue
        freq = base_freq * random.choice([2, 3, 4, 5, 6])
        t = sine_wave(freq, td, sr)[:td_samp] * 0.06
        t *= adsr_envelope(td, sr, a=0.001, d=0.01, s_level=0.3, r=0.05)[:td_samp]
        texture[ts:ts + td_samp] += t

    output = bass + drums + texture
    output = simple_reverb(output, sr, decay=0.15, delay_ms=50)
    output = np.clip(output, -1.0, 1.0)
    output *= 0.85
    return output


# ═══════════════════════════════════════════════════════════════
#  MUSIC GENERATION — MAIN DISPATCHER
# ═══════════════════════════════════════════════════════════════

MUSIC_GENRES = [
    "🎹 Synthwave",
    "🌑 Dark Ambient",
    "🔊 Cyber Bass",
    "💖 Neon Pop",
    "🎛️ Glitch Hop",
]

def generate_synth_music(genre, bpm, duration, key, progress=gr.Progress()):
    random.seed(int(time.time() * 1000) % (2 ** 32))
    np.random.seed(int(time.time() * 1000) % (2 ** 32))

    progress(0.1, desc="🎵 SYNTH CORE BOOTING...")
    time.sleep(0.05)

    genre_map = {
        "🎹 Synthwave": generate_synthwave_music,
        "🌑 Dark Ambient": generate_dark_ambient_music,
        "🔊 Cyber Bass": generate_cyber_bass_music,
        "💖 Neon Pop": generate_neon_pop_music,
        "🎛️ Glitch Hop": generate_glitch_hop_music,
    }

    gen_fn = genre_map.get(genre, generate_synthwave_music)

    progress(0.3, desc="🔮 INITIALIZING OSCILLATORS...")
    progress(0.5, desc="🎼 GENERATING WAVEFORMS...")

    audio = gen_fn(int(bpm), float(duration), key, SR)

    progress(0.8, desc="✨ APPLYING EFFECTS...")
    progress(0.95, desc="💾 ENCODING AUDIO...")

    # Save to temp WAV
    audio_int16 = (audio * 32767).astype(np.int16)
    tmp = tempfile.NamedTemporaryFile(suffix=".wav", delete=False)
    with wave.open(tmp.name, "w") as wf:
        wf.setnchannels(1)
        wf.setsampwidth(2)
        wf.setframerate(SR)
        wf.writeframes(audio_int16.tobytes())

    progress(1.0, desc="🔊 AUDIO MATERIALIZED!")

    log = (
        f"╔══ SYNTH FORGE LOG ═══════════════════════════╗\n"
        f"║ GENRE: {genre}\n"
        f"║ BPM: {int(bpm)}\n"
        f"║ DURATION: {duration:.1f}s\n"
        f"║ KEY: {key}\n"
        f"║ SAMPLE RATE: {SR} Hz\n"
        f"║ SAMPLES: {len(audio):,}\n"
        f"║ STATUS: ███████████████ COMPLETE\n"
        f"╚═════════════════════════════════════════════╝"
    )
    return tmp.name, log


# ═══════════════════════════════════════════════════════════════
#  CODE GENERATION ENGINE
# ═══════════════════════════════════════════════════════════════

CODE_TEMPLATES = {
    ("Python", "Algorithm"): '''# ═══════════════════════════════════════════════════
# CYBERPUNK NEURAL PATHFINDER — Quantum A* Algorithm
# ═══════════════════════════════════════════════════
import heapq
from typing import Dict, List, Tuple, Optional

class NeonGrid:
    """A* pathfinder through the neon-lit cyber grid."""

    def __init__(self, width: int, height: int):
        self.width = width
        self.height = height
        self.barriers: set = set()
        self.neon_zones: Dict[Tuple[int, int], float] = {{}}

    def add_barrier(self, x: int, y: int):
        self.barriers.add((x, y))

    def add_neon_zone(self, x: int, y: int, intensity: float):
        self.neon_zones[(x, y)] = intensity

    def heuristic(self, a: Tuple[int, int], b: Tuple[int, int]) -> float:
        return abs(a[0] - b[0]) + abs(a[1] - b[1])

    def neighbors(self, node: Tuple[int, int]) -> List[Tuple[int, int]]:
        x, y = node
        candidates = [(x+1, y), (x-1, y), (x, y+1), (x, y-1),
                       (x+1, y+1), (x-1, y-1), (x+1, y-1), (x-1, y+1)]
        return [(nx, ny) for nx, ny in candidates
                if 0 <= nx < self.width and 0 <= ny < self.height
                and (nx, ny) not in self.barriers]

    def find_path(self, start: Tuple[int, int],
                  goal: Tuple[int, int]) -> Optional[List[Tuple[int, int]]]:
        frontier = [(0, start)]
        came_from: Dict[Tuple[int, int], Optional[Tuple[int, int]]] = {{start: None}}
        cost_so_far: Dict[Tuple[int, int], float] = {{start: 0}}

        while frontier:
            _, current = heapq.heappop(frontier)
            if current == goal:
                break

            for nxt in self.neighbors(current):
                move_cost = 1.4 if (nxt[0] != current[0] and nxt[1] != current[1]) else 1.0
                neon_bonus = self.neon_zones.get(nxt, 0) * 0.5
                new_cost = cost_so_far[current] + move_cost - neon_bonus

                if nxt not in cost_so_far or new_cost < cost_so_far[nxt]:
                    cost_so_far[nxt] = new_cost
                    priority = new_cost + self.heuristic(nxt, goal)
                    heapq.heappush(frontier, (priority, nxt))
                    came_from[nxt] = current

        if goal not in came_from:
            return None

        path = []
        node = goal
        while node is not None:
            path.append(node)
            node = came_from[node]
        path.reverse()
        return path

# ═══ EXECUTE ═══
if __name__ == "__main__":
    grid = NeonGrid(50, 50)
    for i in range(10, 40):
        grid.add_barrier(25, i)
    grid.add_neon_zone(30, 25, 2.0)
    path = grid.find_path((5, 5), (45, 45))
    print(f"[NEON PATHFINDER] Route found: {{len(path)}} nodes")
    for p in path[:5]:
        print(f"  → {{p}}")
    print("  ...")
''',

    ("Python", "Neural Network"): '''# ═══════════════════════════════════════════════════
# CYBERPUNK DEEP DREAM — Neural Architecture v7.7
# ═══════════════════════════════════════════════════
import numpy as np
from typing import List, Callable

class QuantumActivation:
    """Custom activation functions for the cyber neural core."""

    @staticmethod
    def neon_relu(x: np.ndarray) -> np.ndarray:
        return np.maximum(0.01 * x, x)

    @staticmethod
    def cyber_sigmoid(x: np.ndarray) -> np.ndarray:
        return 1.0 / (1.0 + np.exp(-np.clip(x, -500, 500)))

    @staticmethod
    def glitch_tanh(x: np.ndarray) -> np.ndarray:
        return np.tanh(x) + 0.01 * np.sin(x * 10)

class NeuralLayer:
    def __init__(self, input_dim: int, output_dim: int,
                 activation: Callable = QuantumActivation.neon_relu):
        scale = np.sqrt(2.0 / input_dim)
        self.weights = np.random.randn(input_dim, output_dim) * scale
        self.biases = np.zeros((1, output_dim))
        self.activation = activation
        self.last_input = None
        self.last_output = None

    def forward(self, x: np.ndarray) -> np.ndarray:
        self.last_input = x
        z = x @ self.weights + self.biases
        self.last_output = self.activation(z)
        return self.last_output

class CyberNetwork:
    """Deep neural network — the digital dreamer."""

    def __init__(self, architecture: List[int]):
        self.layers: List[NeuralLayer] = []
        for i in range(len(architecture) - 1):
            act = (QuantumActivation.cyber_sigmoid
                   if i == len(architecture) - 2
                   else QuantumActivation.neon_relu)
            self.layers.append(NeuralLayer(architecture[i], architecture[i + 1], act))

    def forward(self, x: np.ndarray) -> np.ndarray:
        for layer in self.layers:
            x = layer.forward(x)
        return x

    def dream(self, seed: np.ndarray, iterations: int = 10) -> np.ndarray:
        dream_state = seed.copy()
        for i in range(iterations):
            dream_state = self.forward(dream_state)
            noise = np.random.randn(*dream_state.shape) * (0.1 / (i + 1))
            dream_state = dream_state + noise
        return dream_state

# ═══ DREAM SEQUENCE ═══
if __name__ == "__main__":
    network = CyberNetwork([128, 256, 128, 64, 32])
    seed = np.random.randn(1, 128)
    print("[NEURAL CORE] Initiating dream sequence...")
    dream = network.dream(seed, iterations=20)
    print(f"[NEURAL CORE] Dream dimensions: {{dream.shape}}")
    print(f"[NEURAL CORE] Dream intensity: {{dream.mean():.4f}}")
    print("[NEURAL CORE] Dream complete. Reality anchor holding.")
''',

    ("Python", "API Server"): '''# ═══════════════════════════════════════════════════
# CYBERPUNK API NEXUS — FastAPI Server v2.0
# ═══════════════════════════════════════════════════
from fastapi import FastAPI, HTTPException, Depends
from pydantic import BaseModel, Field
from typing import Optional, List
from datetime import datetime
import hashlib
import secrets

app = FastAPI(
    title="Neon Nexus API",
    description="Cyberpunk data gateway — access the grid",
    version="2.0.77",
)

class CyberAgent(BaseModel):
    handle: str = Field(..., min_length=3, max_length=32)
    class_type: str = Field(..., pattern="^(netrunner|solo|fixer|techie)$")
    reputation: int = Field(default=0, ge=0, le=100)
    implants: List[str] = Field(default_factory=list)

class MissionBrief(BaseModel):
    title: str
    danger_level: int = Field(ge=1, le=10)
    reward_credits: float
    description: Optional[str] = None
    required_class: Optional[str] = None

AGENTS_DB: dict = {{}}
MISSIONS_DB: dict = {{}}

def generate_id() -> str:
    return hashlib.sha256(secrets.token_bytes(32)).hexdigest()[:16]

@app.get("/status")
async def system_status():
    return {{
        "status": "ONLINE",
        "grid_integrity": "99.7%",
        "active_agents": len(AGENTS_DB),
        "pending_missions": len(MISSIONS_DB),
        "timestamp": datetime.utcnow().isoformat(),
        "version": "2.0.77",
    }}

@app.post("/agents/register")
async def register_agent(agent: CyberAgent):
    agent_id = generate_id()
    AGENTS_DB[agent_id] = agent.model_dump()
    return {{"id": agent_id, "message": f"Agent {{agent.handle}} jacked into the grid"}}

@app.get("/agents/{{agent_id}}")
async def get_agent(agent_id: str):
    if agent_id not in AGENTS_DB:
        raise HTTPException(404, "Agent not found in the grid")
    return AGENTS_DB[agent_id]

@app.post("/missions/create")
async def create_mission(mission: MissionBrief):
    mission_id = generate_id()
    MISSIONS_DB[mission_id] = mission.model_dump()
    return {{"id": mission_id, "message": "Mission uploaded to darknet"}}

@app.get("/missions")
async def list_missions(danger_min: int = 1, danger_max: int = 10):
    filtered = {{k: v for k, v in MISSIONS_DB.items()
                if danger_min <= v["danger_level"] <= danger_max}}
    return {{"missions": filtered, "count": len(filtered)}}

# uvicorn app:app --host 0.0.0.0 --port 7777 --reload
''',

    ("JavaScript", "Algorithm"): '''// ═══════════════════════════════════════════════════
// CYBERPUNK NEURAL SORT — Quantum Merge Sort Engine
// ═══════════════════════════════════════════════════

class NeonSorter {{
  constructor(compareFn = (a, b) => a - b) {{
    this.compare = compareFn;
    this.operations = 0;
    this.swaps = 0;
  }}

  mergeSort(arr) {{
    this.operations = 0;
    this.swaps = 0;
    const result = this._divide(arr.slice());
    console.log(`[NEON SORT] Ops: ${{this.operations}} | Swaps: ${{this.swaps}}`);
    return result;
  }}

  _divide(arr) {{
    if (arr.length <= 1) return arr;
    this.operations++;
    const mid = Math.floor(arr.length / 2);
    const left = this._divide(arr.slice(0, mid));
    const right = this._divide(arr.slice(mid));
    return this._merge(left, right);
  }}

  _merge(left, right) {{
    const result = [];
    let i = 0, j = 0;
    while (i < left.length && j < right.length) {{
      this.operations++;
      if (this.compare(left[i], right[j]) <= 0) {{
        result.push(left[i++]);
      }} else {{
        result.push(right[j++]);
        this.swaps++;
      }}
    }}
    return result.concat(left.slice(i)).concat(right.slice(j));
  }}

  cyberQuickSort(arr) {{
    if (arr.length <= 1) return arr;
    const pivot = arr[Math.floor(Math.random() * arr.length)];
    const lo = arr.filter(x => this.compare(x, pivot) < 0);
    const eq = arr.filter(x => this.compare(x, pivot) === 0);
    const hi = arr.filter(x => this.compare(x, pivot) > 0);
    return [...this.cyberQuickSort(lo), ...eq, ...this.cyberQuickSort(hi)];
  }}
}}

class DataStream {{
  static generate(size, maxVal = 1000) {{
    return Array.from({{ length: size }}, () => Math.floor(Math.random() * maxVal));
  }}

  static isSorted(arr, compareFn = (a, b) => a - b) {{
    for (let i = 1; i < arr.length; i++) {{
      if (compareFn(arr[i - 1], arr[i]) > 0) return false;
    }}
    return true;
  }}
}}

// ═══ EXECUTE ═══
const sorter = new NeonSorter();
const stream = DataStream.generate(1000);
console.log("[CYBER GRID] Initiating quantum sort...");
const sorted = sorter.mergeSort(stream);
console.log(`[CYBER GRID] Sorted: ${{DataStream.isSorted(sorted)}}`);
console.log(`[CYBER GRID] First 10: ${{sorted.slice(0, 10)}}`);
''',

    ("JavaScript", "Game Engine"): '''// ═══════════════════════════════════════════════════
// NEON ARCADE — Cyberpunk Canvas Game Engine v1.0
// ═══════════════════════════════════════════════════

class NeonEngine {{
  constructor(canvasId, width = 800, height = 600) {{
    this.canvas = document.getElementById(canvasId);
    this.ctx = this.canvas.getContext("2d");
    this.canvas.width = width;
    this.canvas.height = height;
    this.entities = [];
    this.particles = [];
    this.keys = {{}};
    this.running = false;
    this.lastTime = 0;
    this.fps = 0;
    this._initInput();
  }}

  _initInput() {{
    window.addEventListener("keydown", (e) => (this.keys[e.code] = true));
    window.addEventListener("keyup", (e) => (this.keys[e.code] = false));
  }}

  spawn(entity) {{
    this.entities.push(entity);
    return entity;
  }}

  emitParticles(x, y, count = 10, color = "#ff00ff") {{
    for (let i = 0; i < count; i++) {{
      this.particles.push({{
        x, y,
        vx: (Math.random() - 0.5) * 6,
        vy: (Math.random() - 0.5) * 6,
        life: 1.0,
        decay: 0.02 + Math.random() * 0.03,
        color,
        size: 2 + Math.random() * 3,
      }});
    }}
  }}

  update(dt) {{
    this.entities.forEach((e) => e.update?.(dt, this));
    this.particles.forEach((p) => {{
      p.x += p.vx;
      p.y += p.vy;
      p.life -= p.decay;
    }});
    this.particles = this.particles.filter((p) => p.life > 0);
    this.entities = this.entities.filter((e) => !e.destroyed);
  }}

  render() {{
    const {{ ctx, canvas }} = this;
    ctx.fillStyle = "rgba(5, 0, 15, 0.3)";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
    this.particles.forEach((p) => {{
      ctx.globalAlpha = p.life;
      ctx.fillStyle = p.color;
      ctx.beginPath();
      ctx.arc(p.x, p.y, p.size * p.life, 0, Math.PI * 2);
      ctx.fill();
    }});
    ctx.globalAlpha = 1;
    this.entities.forEach((e) => e.render?.(ctx));
    ctx.fillStyle = "#00ffff";
    ctx.font = "12px monospace";
    ctx.fillText(`FPS: ${{this.fps}} | ENTITIES: ${{this.entities.length}}`, 10, 20);
  }}

  loop(timestamp) {{
    if (!this.running) return;
    const dt = (timestamp - this.lastTime) / 1000;
    this.fps = Math.round(1 / dt);
    this.lastTime = timestamp;
    this.update(dt);
    this.render();
    requestAnimationFrame((t) => this.loop(t));
  }}

  start() {{
    this.running = true;
    this.lastTime = performance.now();
    requestAnimationFrame((t) => this.loop(t));
    console.log("[NEON ENGINE] Grid online. Game loop active.");
  }}
}}

// ═══ USAGE ═══
// const engine = new NeonEngine("gameCanvas");
// engine.start();
''',

    ("Rust", "Algorithm"): '''// ═══════════════════════════════════════════════════
// CYBERPUNK HASH MATRIX — Quantum Hash Map in Rust
// ═══════════════════════════════════════════════════

use std::collections::hash_map::DefaultHasher;
use std::hash::{{Hash, Hasher}};

const NEON_CAPACITY: usize = 64;
const LOAD_THRESHOLD: f64 = 0.75;

#[derive(Clone)]
enum Slot<K, V> {{
    Empty,
    Occupied(K, V),
    Deleted,
}}

struct CyberMap<K, V> {{
    slots: Vec<Slot<K, V>>,
    capacity: usize,
    len: usize,
}}

impl<K: Hash + Eq + Clone, V: Clone> CyberMap<K, V> {{
    fn new() -> Self {{
        CyberMap {{
            slots: vec![Slot::Empty; NEON_CAPACITY],
            capacity: NEON_CAPACITY,
            len: 0,
        }}
    }}

    fn hash_index(&self, key: &K) -> usize {{
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }}

    fn insert(&mut self, key: K, value: V) -> Option<V> {{
        if (self.len as f64 / self.capacity as f64) > LOAD_THRESHOLD {{
            self.grow();
        }}
        let mut idx = self.hash_index(&key);
        loop {{
            match &self.slots[idx] {{
                Slot::Empty | Slot::Deleted => {{
                    self.slots[idx] = Slot::Occupied(key, value);
                    self.len += 1;
                    return None;
                }}
                Slot::Occupied(k, _) if k == &key => {{
                    let old = if let Slot::Occupied(_, v) = &self.slots[idx] {{
                        Some(v.clone())
                    }} else {{
                        None
                    }};
                    self.slots[idx] = Slot::Occupied(key, value);
                    return old;
                }}
                _ => idx = (idx + 1) % self.capacity,
            }}
        }}
    }}

    fn get(&self, key: &K) -> Option<&V> {{
        let mut idx = self.hash_index(key);
        let start = idx;
        loop {{
            match &self.slots[idx] {{
                Slot::Empty => return None,
                Slot::Occupied(k, v) if k == key => return Some(v),
                _ => {{
                    idx = (idx + 1) % self.capacity;
                    if idx == start {{ return None; }}
                }}
            }}
        }}
    }}

    fn grow(&mut self) {{
        let new_cap = self.capacity * 2;
        let old_slots = std::mem::replace(
            &mut self.slots, vec![Slot::Empty; new_cap]
        );
        self.capacity = new_cap;
        self.len = 0;
        for slot in old_slots {{
            if let Slot::Occupied(k, v) = slot {{
                self.insert(k, v);
            }}
        }}
    }}

    fn len(&self) -> usize {{ self.len }}
}}

fn main() {{
    let mut grid = CyberMap::new();
    grid.insert("netrunner", 42);
    grid.insert("ice_breaker", 77);
    grid.insert("daemon", 13);

    println!("[CYBER MAP] Grid size: {{}}", grid.len());
    if let Some(v) = grid.get(&"netrunner") {{
        println!("[CYBER MAP] netrunner → {{}}", v);
    }}
    println!("[CYBER MAP] Data matrix operational.");
}}
''',

    ("Rust", "Systems"): '''// ═══════════════════════════════════════════════════
// NEON THREAD POOL — Concurrent Task Engine in Rust
// ═══════════════════════════════════════════════════

use std::sync::{{mpsc, Arc, Mutex}};
use std::thread;

type NeonTask = Box<dyn FnOnce() + Send + 'static>;

struct CyberPool {{
    workers: Vec<thread::JoinHandle<()>>,
    sender: Option<mpsc::Sender<NeonTask>>,
}}

impl CyberPool {{
    fn new(size: usize) -> Self {{
        let (sender, receiver) = mpsc::channel::<NeonTask>();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {{
            let rx = Arc::clone(&receiver);
            let handle = thread::spawn(move || {{
                loop {{
                    let task = rx.lock().unwrap().recv();
                    match task {{
                        Ok(job) => {{
                            println!("[WORKER-{{}}] Executing neon task...", id);
                            job();
                        }}
                        Err(_) => {{
                            println!("[WORKER-{{}}] Grid disconnected. Shutting down.", id);
                            break;
                        }}
                    }}
                }}
            }});
            workers.push(handle);
        }}

        CyberPool {{
            workers,
            sender: Some(sender),
        }}
    }}

    fn execute<F: FnOnce() + Send + 'static>(&self, task: F) {{
        if let Some(ref sender) = self.sender {{
            sender.send(Box::new(task)).expect("Grid send failure");
        }}
    }}
}}

impl Drop for CyberPool {{
    fn drop(&mut self) {{
        drop(self.sender.take());
        for worker in self.workers.drain(..) {{
            worker.join().expect("Worker panic detected");
        }}
        println!("[NEON POOL] All workers terminated. Grid offline.");
    }}
}}

fn main() {{
    println!("[NEON POOL] Initializing 4-core cyber grid...");
    let pool = CyberPool::new(4);

    for i in 0..8 {{
        pool.execute(move || {{
            let result: u64 = (0..1_000_000).sum();
            println!("[TASK-{{}}] Computation complete: {{}}", i, result);
        }});
    }}

    drop(pool);
    println!("[NEON POOL] All tasks complete. Grid shutdown.");
}}
''',

    ("Go", "Algorithm"): '''// ═══════════════════════════════════════════════════
// CYBERPUNK GRAPH MATRIX — BFS/DFS Engine in Go
// ═══════════════════════════════════════════════════
package main

import "fmt"

type NeonGraph struct {{
    adjacency map[string][]string
    weights   map[string]map[string]float64
}}

func NewNeonGraph() *NeonGraph {{
    return &NeonGraph{{
        adjacency: make(map[string][]string),
        weights:   make(map[string]map[string]float64),
    }}
}}

func (g *NeonGraph) AddEdge(from, to string, weight float64) {{
    g.adjacency[from] = append(g.adjacency[from], to)
    if g.weights[from] == nil {{
        g.weights[from] = make(map[string]float64)
    }}
    g.weights[from][to] = weight
}}

func (g *NeonGraph) BFS(start string) []string {{
    visited := make(map[string]bool)
    queue := []string{{start}}
    visited[start] = true
    var order []string

    for len(queue) > 0 {{
        node := queue[0]
        queue = queue[1:]
        order = append(order, node)
        for _, neighbor := range g.adjacency[node] {{
            if !visited[neighbor] {{
                visited[neighbor] = true
                queue = append(queue, neighbor)
            }}
        }}
    }}
    return order
}}

func (g *NeonGraph) DFS(start string) []string {{
    visited := make(map[string]bool)
    var order []string
    g.dfsHelper(start, visited, &order)
    return order
}}

func (g *NeonGraph) dfsHelper(node string, visited map[string]bool, order *[]string) {{
    visited[node] = true
    *order = append(*order, node)
    for _, neighbor := range g.adjacency[node] {{
        if !visited[neighbor] {{
            g.dfsHelper(neighbor, visited, order)
        }}
    }}
}}

func main() {{
    grid := NewNeonGraph()
    grid.AddEdge("NeoTokyo", "ChronoDistrict", 2.5)
    grid.AddEdge("NeoTokyo", "NeonAlley", 1.2)
    grid.AddEdge("ChronoDistrict", "DataVault", 3.1)
    grid.AddEdge("NeonAlley", "SynthPlaza", 0.8)
    grid.AddEdge("SynthPlaza", "DataVault", 1.5)
    grid.AddEdge("DataVault", "CyberCore", 4.0)

    fmt.Println("[NEON GRAPH] BFS traversal from NeoTokyo:")
    for _, node := range grid.BFS("NeoTokyo") {{
        fmt.Printf("  → %s\\n", node)
    }}

    fmt.Println("[NEON GRAPH] DFS traversal from NeoTokyo:")
    for _, node := range grid.DFS("NeoTokyo") {{
        fmt.Printf("  → %s\\n", node)
    }}
}}
''',

    ("Go", "API Server"): '''// ═══════════════════════════════════════════════════
// NEON NEXUS — Cyberpunk HTTP Server in Go
// ═══════════════════════════════════════════════════
package main

import (
    "encoding/json"
    "fmt"
    "log"
    "net/http"
    "sync"
    "time"
)

type Agent struct {{
    Handle    string   `json:"handle"`
    ClassType string   `json:"class_type"`
    Rep       int      `json:"reputation"`
    Implants  []string `json:"implants"`
}}

type GridStatus struct {{
    Status     string `json:"status"`
    Agents     int    `json:"active_agents"`
    Uptime     string `json:"uptime"`
    Version    string `json:"version"`
}}

var (
    agents   = make(map[string]Agent)
    agentsMu sync.RWMutex
    startTime = time.Now()
)

func statusHandler(w http.ResponseWriter, r *http.Request) {{
    agentsMu.RLock()
    count := len(agents)
    agentsMu.RUnlock()

    status := GridStatus{{
        Status:  "ONLINE",
        Agents:  count,
        Uptime:  time.Since(startTime).String(),
        Version: "2.0.77",
    }}
    w.Header().Set("Content-Type", "application/json")
    json.NewEncoder(w).Encode(status)
}}

func registerHandler(w http.ResponseWriter, r *http.Request) {{
    if r.Method != http.MethodPost {{
        http.Error(w, "METHOD_NOT_ALLOWED", http.StatusMethodNotAllowed)
        return
    }}
    var agent Agent
    if err := json.NewDecoder(r.Body).Decode(&agent); err != nil {{
        http.Error(w, "MALFORMED_DATA", http.StatusBadRequest)
        return
    }}
    agentsMu.Lock()
    agents[agent.Handle] = agent
    agentsMu.Unlock()

    w.Header().Set("Content-Type", "application/json")
    json.NewEncoder(w).Encode(map[string]string{{
        "status":  "registered",
        "message": fmt.Sprintf("Agent %s jacked in", agent.Handle),
    }})
}}

func main() {{
    mux := http.NewServeMux()
    mux.HandleFunc("/status", statusHandler)
    mux.HandleFunc("/agents/register", registerHandler)

    addr := ":7777"
    log.Printf("[NEON NEXUS] Grid online at %s", addr)
    log.Fatal(http.ListenAndServe(addr, mux))
}}
''',
}

CODE_LANGUAGES = ["Python", "JavaScript", "Rust", "Go"]
CODE_TYPES = {
    "Python": ["Algorithm", "Neural Network", "API Server"],
    "JavaScript": ["Algorithm", "Game Engine"],
    "Rust": ["Algorithm", "Systems"],
    "Go": ["Algorithm", "API Server"],
}


def generate_code_matrix(language, code_type, progress=gr.Progress()):
    progress(0.1, desc="💻 CODE MATRIX COMPILING...")
    time.sleep(0.05)

    progress(0.3, desc="🔮 LOADING CYBER TEMPLATES...")

    key = (language, code_type)
    code = CODE_TEMPLATES.get(key, f"// [{language}:{code_type}] Template not found in the matrix.\n// Check back when the grid expands.")

    progress(0.7, desc="⚡ INJECTING NEON SYNTAX...")
    progress(1.0, desc="💾 CODE FORGED!")

    log = (
        f"╔══ CODE MATRIX LOG ═══════════════════════════╗\n"
        f"║ LANGUAGE: {language}\n"
        f"║ TYPE: {code_type}\n"
        f"║ LINES: {len(code.splitlines())}\n"
        f"║ SIZE: {len(code)} bytes\n"
        f"║ STATUS: ███████████████ COMPLETE\n"
        f"╚═════════════════════════════════════════════╝"
    )
    return code, log


def update_code_types(language):
    types = CODE_TYPES.get(language, ["Algorithm"])
    return gr.update(choices=types, value=types[0])


# ═══════════════════════════════════════════════════════════════
#  GRADIO UI — THE NEON INTERFACE
# ═══════════════════════════════════════════════════════════════

def build_app():
    with gr.Blocks(
        css=CYBERPUNK_CSS,
        title="CYBERPUNK DREAM GENERATOR",
        theme=gr.themes.Base(
            primary_hue=gr.themes.colors.fuchsia,
            secondary_hue=gr.themes.colors.cyan,
            neutral_hue=gr.themes.colors.slate,
            font=[gr.themes.GoogleFont("Rajdhani"), "system-ui", "sans-serif"],
            font_mono=[gr.themes.GoogleFont("Share Tech Mono"), "ui-monospace", "monospace"],
        ),
    ) as demo:

        # ═══ HEADER ═══
        gr.HTML(HEADER_HTML)

        # ═══ MAIN TABS ═══
        with gr.Tabs(elem_classes="cyber-tabs"):

            # ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
            #  TAB 1: DREAM ART LAB
            # ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
            with gr.TabItem("🎨 DREAM ART LAB", id="art"):
                with gr.Row():
                    with gr.Column(scale=1, min_width=320):
                        gr.HTML('<div class="section-header">◈ DREAM PARAMETERS ◈</div>')

                        art_prompt = gr.Textbox(
                            label="DREAM PROMPT",
                            placeholder="Enter your dream vision... (e.g. neon city rain at midnight)",
                            lines=3,
                            elem_classes="cyber-input",
                        )
                        art_style = gr.Dropdown(
                            choices=ART_STYLES,
                            value=ART_STYLES[0],
                            label="ART STYLE",
                        )
                        art_palette = gr.Dropdown(
                            choices=list(PALETTES.keys()),
                            value="🔮 Neon Noir",
                            label="COLOR PALETTE",
                        )
                        with gr.Row():
                            art_width = gr.Slider(256, 1024, 768, step=64, label="WIDTH")
                            art_height = gr.Slider(256, 1024, 768, step=64, label="HEIGHT")
                        art_intensity = gr.Slider(0.3, 2.0, 1.0, step=0.1, label="NEON INTENSITY")
                        art_btn = gr.Button("⚡ GENERATE DREAM", variant="primary", size="lg")

                        gr.Examples(
                            examples=[
                                ["neon rain cyberpunk alley at midnight", "🏙️ Cyber City", "🔮 Neon Noir"],
                                ["digital consciousness awakening", "💫 Digital Dreams", "🌊 Cyber Ocean"],
                                ["synthwave sunset drive forever", "🌅 Synthwave Sunset", "🔥 Digital Inferno"],
                                ["quantum entanglement visualization", "🔮 Quantum Realm", "⚡ Electric Storm"],
                                ["neural network dreaming of electric sheep", "🧠 Neural Network", "🌿 Matrix Green"],
                                ["broken reality glitch in the matrix", "📺 Glitch Art", "🌈 Rainbow Glitch"],
                            ],
                            inputs=[art_prompt, art_style, art_palette],
                            label="⚡ PRESET DREAMS",
                            elem_classes="examples-row",
                        )

                    with gr.Column(scale=2):
                        gr.HTML('<div class="section-header">◈ DREAM OUTPUT ◈</div>')
                        art_output = gr.Image(label="GENERATED DREAM", type="pil", height=550)
                        art_log = gr.Textbox(label="NEURAL LOG", lines=9, interactive=False, elem_classes="cyber-log")

            # ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
            #  TAB 2: SYNTH MUSIC FORGE
            # ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
            with gr.TabItem("🎵 SYNTH MUSIC FORGE", id="music"):
                with gr.Row():
                    with gr.Column(scale=1, min_width=320):
                        gr.HTML('<div class="section-header">◈ SYNTH PARAMETERS ◈</div>')

                        music_genre = gr.Dropdown(
                            choices=MUSIC_GENRES,
                            value=MUSIC_GENRES[0],
                            label="GENRE",
                        )
                        music_key = gr.Dropdown(
                            choices=SCALE_NOTES,
                            value="A",
                            label="KEY",
                        )
                        music_bpm = gr.Slider(60, 200, 120, step=5, label="BPM")
                        music_duration = gr.Slider(3, 30, 10, step=1, label="DURATION (seconds)")
                        music_btn = gr.Button("🎵 FORGE TRACK", variant="primary", size="lg")

                        gr.Examples(
                            examples=[
                                ["🎹 Synthwave", 110, "A"],
                                ["🌑 Dark Ambient", 70, "D"],
                                ["🔊 Cyber Bass", 150, "E"],
                                ["💖 Neon Pop", 128, "C"],
                                ["🎛️ Glitch Hop", 105, "F#"],
                            ],
                            inputs=[music_genre, music_bpm, music_key],
                            label="⚡ PRESET GENRES",
                            elem_classes="examples-row",
                        )

                    with gr.Column(scale=2):
                        gr.HTML('<div class="section-header">◈ AUDIO OUTPUT ◈</div>')
                        music_output = gr.Audio(label="GENERATED TRACK", type="filepath")
                        music_log = gr.Textbox(label="SYNTH LOG", lines=9, interactive=False, elem_classes="cyber-log")

            # ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
            #  TAB 3: CODE MATRIX
            # ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
            with gr.TabItem("💻 CODE MATRIX", id="code"):
                with gr.Row():
                    with gr.Column(scale=1, min_width=320):
                        gr.HTML('<div class="section-header">◈ CODE PARAMETERS ◈</div>')

                        code_lang = gr.Dropdown(
                            choices=CODE_LANGUAGES,
                            value="Python",
                            label="LANGUAGE",
                        )
                        code_type = gr.Dropdown(
                            choices=CODE_TYPES["Python"],
                            value="Algorithm",
                            label="CODE TYPE",
                        )
                        code_btn = gr.Button("💻 FORGE CODE", variant="primary", size="lg")

                        gr.Examples(
                            examples=[
                                ["Python", "Algorithm"],
                                ["Python", "Neural Network"],
                                ["Python", "API Server"],
                                ["JavaScript", "Game Engine"],
                                ["Rust", "Systems"],
                                ["Go", "API Server"],
                            ],
                            inputs=[code_lang, code_type],
                            label="⚡ PRESET TEMPLATES",
                            elem_classes="examples-row",
                        )

                    with gr.Column(scale=2):
                        gr.HTML('<div class="section-header">◈ CODE OUTPUT ◈</div>')
                        code_output = gr.Code(label="GENERATED CODE", language="python", lines=30)
                        code_log = gr.Textbox(label="COMPILE LOG", lines=7, interactive=False, elem_classes="cyber-log")

            # ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
            #  TAB 4: SYSTEM CORE
            # ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
            with gr.TabItem("⚙️ SYSTEM CORE", id="system"):
                gr.HTML(SYSTEM_CORE_HTML)

        # ═══ FOOTER ═══
        gr.HTML(FOOTER_HTML)

        # ═══ EVENT HANDLERS ═══
        art_btn.click(
            fn=generate_dream_art,
            inputs=[art_prompt, art_style, art_width, art_height, art_palette, art_intensity],
            outputs=[art_output, art_log],
        )

        music_btn.click(
            fn=generate_synth_music,
            inputs=[music_genre, music_bpm, music_duration, music_key],
            outputs=[music_output, music_log],
        )

        code_btn.click(
            fn=generate_code_matrix,
            inputs=[code_lang, code_type],
            outputs=[code_output, code_log],
        )

        code_lang.change(
            fn=update_code_types,
            inputs=[code_lang],
            outputs=[code_type],
        )

    return demo


# ═══════════════════════════════════════════════════════════════
#  LAUNCH — IGNITE THE NEON GRID
# ═══════════════════════════════════════════════════════════════

if __name__ == "__main__":
    demo = build_app()
    demo.launch()
