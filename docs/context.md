# AI Screen Assistant - Project Overview

## Core Concept
A desktop AI assistant that you activate with a hotkey, can see your screen via screenshots, and helps with whatever you're working on. Think "instant AI anywhere" with visual context.

## Key Features
- **Hotkey activation** - Press hotkey → AI pops up instantly, sees current screen
- **Visual context** - AI analyzes screenshots to understand what you're looking at
- **Interactive context building** - When AI needs more info: "Can you show me your data source?" → You navigate → Press hotkey again → Sends another screenshot
- **Read-only approach** - AI can see but not control your screen (safer, more trustworthy)
- **Universal compatibility** - Works with any application (Excel, Figma, VS Code, etc.)

## User Experience Flow
1. User encounters problem while working
2. Press hotkey (e.g., Cmd+Shift+A)
3. AI assistant appears, already seeing current screen
4. User asks: "Why isn't this working?"
5. AI analyzes screenshot, provides help
6. If needed: AI asks "Can you show me the formula in cell B5?"
7. User navigates → Press hotkey → AI gets additional context
8. Conversation continues with rich visual context

## Privacy & Memory Solution
### Privacy-First Approach
- Screenshots sent to AI provider, **deleted immediately after processing**
- User controls exactly what gets shared via hotkey presses
- No continuous monitoring or background screen recording
- End-to-end encryption for data in transit

### Smart Memory System
- **Local storage only** - All learning/memory stored on user's device
- **Explicit consent** - AI asks permission before remembering anything
- **Transparent saving** - "Can I remember that you prefer React hooks over class components?"
- **User-controlled** - Users decide what gets remembered vs automatic collection

Examples of what gets remembered locally:
- Coding preferences and patterns
- Workflow preferences
- Tool configurations
- Project context (if approved)

## Use Cases That Save Real Time

### Excel/Spreadsheet Debugging
- **Problem**: Pivot table broken, formula errors, data analysis issues
- **Current flow**: Screenshot → Upload to ChatGPT → Explain context → Wait
- **Our flow**: Hotkey → "Fix this" → Instant analysis → Follow-up screenshots as needed
- **Time saved**: 30-60 seconds per question, adds up over long sessions

### Code Debugging
- **Problem**: Bug hunting, error analysis, code review
- **Advantage**: AI sees error + code + console simultaneously
- **Value**: Spots obvious mistakes you've looked at 10 times but missed

### Learning New Software
- **Problem**: First time using Blender/Figma/complex tools
- **Value**: AI sees exact interface → gives specific next steps
- **Time saved**: Minutes per question vs generic tutorial advice

### Design Feedback
- **Problem**: Layout issues, color problems, visual hierarchy
- **Value**: AI spots inconsistencies, contrast issues, alignment problems
- **Advantage**: Like having a senior designer review in real-time

## Competitive Landscape

### Microsoft Copilot
- **Approach**: Alt+Space hotkey + click glasses icon to share screen
- **Limitations**: More friction (multiple clicks), tied to Microsoft ecosystem, Windows-only
- **Data**: Sends to Microsoft servers

### Wagoo
- **Approach**: Similar concept with hotkey activation and screen context
- **Focus**: Privacy-focused with local processing
- **Status**: Recently launched, market validation TBD

### Our Advantages
- **Lower friction**: Single hotkey vs multiple clicks
- **Cross-platform**: Mac, Windows, Linux from day one
- **Better privacy**: Explicit screenshot sharing + local memory
- **Specialized modes**: Domain-specific understanding (coding, design, data analysis)

## Technical Architecture

### AI Model Strategy - Hybrid Approach
**Tier 1: Gemini 2.5 Flash-Lite (90% of interactions)**
- Quick screenshot analysis, simple questions
- Ultra-low cost for high-volume usage
- Input: $0.10/1M tokens, Output: $0.40/1M tokens

**Tier 2: o4-mini (Complex reasoning when needed)**
- Multi-step problems, debugging, complex analysis
- Input: $1.10/1M tokens, Output: $4.40/1M tokens
- Triggered by keywords like "debug", "analyze", "why isn't this working?"

### Cost Analysis (Per Power User - 200 interactions/day)

**With 10% escalation to o4-mini:**
- 180 simple interactions (Flash-Lite): $0.070/day
- 20 complex interactions (o4-mini): $0.046/day
- **Total: $3.48/month per power user**

**Screenshot token costs:**
- 1920x1080 screenshot: ~1,500-2,300 tokens
- Plus user question + system prompt: ~200-550 tokens
- AI response: ~400 tokens average
- **Total per interaction: ~$0.00039 (Flash-Lite)**

## Business Model

### Pricing Tiers
- **Basic ($10/month)**: Up to 50 interactions/day, Gemini only
- **Pro ($30/month)**: Up to 200 interactions/day + o4-mini reasoning
- **Enterprise ($60/month)**: Unlimited + priority processing + team features

### Unit Economics (Pro Tier)
- Revenue: $30/month
- API costs: ~$3.50/month
- **Gross margin: 88%**

### Market Size Validation
- Multiple companies building similar tools (Microsoft, Wagoo)
- Growing demand for AI-integrated workflows
- Universal applicability across all software/workflows

## Key Differentiators

1. **Convenience Factor**: Instant AI anywhere with zero context switching
2. **Privacy-First**: User-controlled sharing + local memory storage
3. **Hybrid Intelligence**: Cheap AI for simple tasks, powerful AI for complex reasoning
4. **Cross-Platform**: Not tied to any ecosystem
5. **Visual Context**: Actually sees what you're working on vs text-based context

## Success Metrics
- **Engagement**: Daily interaction count per user
- **Retention**: Weekly/monthly active users
- **Value demonstration**: Time saved per interaction
- **Privacy trust**: User adoption of memory features vs privacy concerns

## Next Steps
1. **MVP Development**: Basic hotkey + screenshot + AI response
2. **Model Integration**: Start with Gemini 2.5 Flash-Lite
3. **Privacy Implementation**: Local memory storage + explicit consent UI
4. **Beta Testing**: Focus on power users (developers, designers, analysts)
5. **Hybrid Model**: Add o4-mini escalation based on usage patterns
6. **Cross-Platform**: Expand beyond initial platform

---

*This represents a focused, privacy-conscious approach to AI-assisted computing that prioritizes user control, convenience, and genuine value creation over flashy features.*