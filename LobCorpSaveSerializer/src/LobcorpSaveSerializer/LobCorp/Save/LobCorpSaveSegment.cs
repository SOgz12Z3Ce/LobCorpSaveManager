using System.Collections.Generic;
using LobCorp.Exceptions;
using Newtonsoft.Json;

namespace LobCorp.Save
{
	public class LobCorpSaveSegment
	{
		private readonly Dictionary<string, object> save;
		private SaveType? _type;
		public LobCorpSaveSegment(Dictionary<string, object> save)
		{
			this.save = save;
		}
		public enum SaveType
		{
			Settings,
			Etc,
			Master,
			Global,
			Unlimited,
			WhiteNight,
		}
		public SaveType Type
		{
			get
			{
				if (_type == null)
				{
					_type = InferType(save);
				}
				return _type.Value;
			}
		}
		private static SaveType InferType(Dictionary<string, object> save)
		{
			if (save.ContainsKey("bgmVolume"))
			{
				return SaveType.Settings;
			}
			if (save.ContainsKey("sefirabossTutorialPlayed"))
			{
				return SaveType.Etc;
			}
			if (save.ContainsKey("playTime"))
			{
				return SaveType.Master;
			}
			if (save.ContainsKey("observe"))
			{
				return SaveType.Global;
			}
			if (save.ContainsKey("money"))
			{
				return SaveType.Unlimited;
			}
			if (save.ContainsKey("genDay"))
			{
				return SaveType.WhiteNight;
			}
			throw new BadSaveFile(string.Format("Unable to infer save type."));
		}
		public string ToJson()
		{
			return JsonConvert.SerializeObject(save);
		}
		public string ToJson(Formatting formatting)
		{
			return JsonConvert.SerializeObject(save, formatting);
		}
	}
}
