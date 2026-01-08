using System;
using System.Collections.Generic;
using LobCorp.Exceptions;
using Newtonsoft.Json.Linq;

namespace LobCorp.Save.Type
{
	public static class SaveTypeInferer
	{
		private static readonly Dictionary<string, SaveType> map = new Dictionary<string, SaveType>()
		{
			{ "bgmVolume", SaveType.Settings },
			{ "sefirabossTutorialPlayed", SaveType.Etc },
			{ "playTime", SaveType.Master },
			{ "observe", SaveType.Global },
			{ "money", SaveType.Unlimited },
			{ "genDay", SaveType.WhiteNight },
		};
		public static SaveType Infer(Dictionary<string, object> save)
		{
			return Infer(save.ContainsKey);
		}
		public static SaveType Infer(JObject save)
		{
			return Infer(save.ContainsKey);
		}
		private static SaveType Infer(Func<string, bool> ContainsKey)
		{
			foreach (var kvp in map)
			{
				if (ContainsKey(kvp.Key))
				{
					return kvp.Value;
				}
			}
			throw new BadSaveFile("Unable to infer save type.");
		}
	}
}
