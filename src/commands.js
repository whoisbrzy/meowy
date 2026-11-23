const { SlashCommandBuilder } = require('discord.js');
const commands = [
 { name:'ping', data:new SlashCommandBuilder().setName('ping').setDescription("See Meowy's latency."),
   execute: i => i.reply(`Pong! ${i.client.ws.ping}ms`) },
 { name:'echo', data:new SlashCommandBuilder().setName('echo').setDescription('Echo a message back to you').addStringOption(o=>o.setName('text').setDescription('Text to echo').setRequired(true)),
   execute: i => i.reply({content:i.options.getString('text'), allowedMentions:{parse:[]}}) },
 { name:'coinflip', data:new SlashCommandBuilder().setName('coinflip').setDescription('Flip one or more coins.').addIntegerOption(o=>o.setName('rounds').setDescription('Number of flips (1–20)').setMinValue(1).setMaxValue(20)),
   execute: i => { const n=i.options.getInteger('rounds')||1; const flips=Array.from({length:n},()=>Math.random()<.5?'Heads':'Tails'); return i.reply(flips.join(', ')); } },
 { name:'color', data:new SlashCommandBuilder().setName('color').setDescription('Preview a color by hex value.').addStringOption(o=>o.setName('value').setDescription('Hex color, e.g. #5865F2').setRequired(true)),
   execute: i => { const v=i.options.getString('value').trim(); if(!/^#?[0-9a-fA-F]{6}$/.test(v)) return i.reply({content:'Please enter a 6-digit hex color.',ephemeral:true}); const hex=v.replace('#',''); return i.reply({embeds:[{color:parseInt(hex,16),title:'#'+hex.toUpperCase(),description:'Color preview'}]}); } },
 { name:'ai', data:new SlashCommandBuilder().setName('ai').setDescription('AI tools').addSubcommand(s=>s.setName('ask').setDescription('Ask AI a question').addStringOption(o=>o.setName('question').setDescription('Question').setRequired(true))).addSubcommand(s=>s.setName('usage').setDescription('View your daily usage')),
   execute: async i => { if(i.options.getSubcommand()==='usage') return i.reply('AI usage tracking will be shown here once an AI provider is configured.'); if(!process.env.AI_API_KEY) return i.reply({content:'AI provider is not configured yet. Add AI_API_KEY and AI_BASE_URL to .env.',ephemeral:true}); return i.reply({content:'AI provider adapter is not yet configured.',ephemeral:true}); } }
];
module.exports={commands};
