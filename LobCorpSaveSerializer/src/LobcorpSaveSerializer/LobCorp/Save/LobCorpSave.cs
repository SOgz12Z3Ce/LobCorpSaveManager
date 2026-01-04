using System.Collections.Generic;
using Newtonsoft.Json;

namespace LobCorp.Save
{
	public class LobCorpSave
	{
		private readonly Dictionary<string, object> save;
		public LobCorpSave(Dictionary<string, object> save)
		{
			this.save = save;
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
