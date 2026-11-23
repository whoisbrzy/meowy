require('dotenv').config();
const { Client, GatewayIntentBits, REST, Routes } = require('discord.js');
const { commands } = require('./commands');
const client = new Client({ intents: [GatewayIntentBits.Guilds] });
const token = process.env.DISCORD_TOKEN;
if (!token) throw new Error('DISCORD_TOKEN is required');
client.once('ready', async () => {
 console.log(`Meowy online as ${client.user.tag}`);
 try {
  const rest = new REST({ version: '10' }).setToken(token);
  await rest.put(Routes.applicationCommands(client.user.id), { body: commands.map(c=>c.data.toJSON()) });
  console.log('Registered slash commands globally.');
 } catch (error) { console.error('Command registration failed:', error); }
});
client.on('interactionCreate', async interaction=>{
 if(!interaction.isChatInputCommand()) return;
 const command=commands.find(c=>c.data.name===interaction.commandName);
 if(!command) return;
 try { await command.execute(interaction); } catch(e){console.error(e);const {reply}=require('./ui');try{await reply(interaction,'An error occurred while running this command.');}catch{}}
});
client.login(token);
